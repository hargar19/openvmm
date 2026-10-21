#!/usr/bin/python3

# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

import os
import sys
import io
import json
import gzip
import shutil
import struct
import subprocess
import tempfile
import time

from gen_init_ramfs import create_cpio_from_config, create_cpio_from_dir
from typing import List

package_id = "Microsoft.HCL.Kernel"

class Config:
    VERBOSE = False
    CPIO = "cpio"
    EXECSTACK = "execstack"
    GZIP = "gzip"
    ZSTD = "zstd"
    BINWALK = "binwalk"
    LSINITRAMFS = "lsinitramfs"
    REQUIRED_TOOLS = [
        CPIO,
        GZIP,
        BINWALK,
        LSINITRAMFS
    ] if VERBOSE else [
    ]

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

def verbose_print(*args, **kwargs):
    if Config.VERBOSE:
        eprint(*args, **kwargs)

def get_script_path():
    return os.path.dirname(os.path.realpath(sys.argv[0]))


def run_inside_shell_and_check(command: str):
    verbose_print(f'Running "{command}"...')
    subprocess.run(
        command,
        shell=True,
        check=True)


def run_and_get_stdout(command: str) -> str:
    verbose_print(f'Running "{command}"...')
    result = subprocess.run(
        command,
        shell=True,
        check=True,
        stdout=subprocess.PIPE)

    return result.stdout.decode("utf-8").strip()


def append_to_rootfs(initial_dir: str, file_to_append: str, existing_cpio_gz_path: str):
    command = f'cd {initial_dir}; ' + \
              f'echo {file_to_append} | ' + \
              f'{Config.CPIO} -o -H newc | {Config.GZIP} >> {existing_cpio_gz_path}'
    run_inside_shell_and_check(command)


def append_file(path: str, append: str):
    command = f"cat {append} >> {path}"
    run_inside_shell_and_check(command)


def generate_build_info(path: str):
    branch = run_and_get_stdout(f"git rev-parse --abbrev-ref HEAD")
    revision = run_and_get_stdout(f"git rev-parse HEAD")
    build_data = {
        "git_branch": branch,
        "git_revision": revision
    }

    with io.open(path, 'w', encoding='utf-8') as f:
        f.write(json.dumps(build_data, ensure_ascii=False, indent=4))


def read_layer_as_cpio(path: str) -> bytes:
    with open(path, 'rb') as f:
        data = f.read()
    if data[:2] == b'\x1f\x8b':
        return gzip.decompress(data)
    if data[:6] == b'070701':
        return data
    raise Exception(f"Layer '{path}' must be a gzip-compressed or uncompressed newc cpio archive")


def build_zstd_initrd(temp_dir: str, rootfs_config_path: List[str], updated_initramfs_path: str,
                      additional_layers: List[str], additional_dirs: List[str]):
    cpio_path = os.path.join(temp_dir, 'underhill.cpio')
    stat = create_cpio_from_config(rootfs_config_path, cpio_path, 'none')
    eprint(f"The initial root fs size, bytes: {stat}")

    # Linux requires each concatenated archive to start 4-byte aligned.
    def append_cpio(out, data: bytes):
        out.write(b'\0' * (-out.tell() % 4))
        out.write(data)

    with open(cpio_path, 'ab') as out:
        for dir_name in additional_dirs:
            dir_cpio = os.path.join(temp_dir, "add_dir.cpio")
            create_cpio_from_dir(dir_name, dir_cpio, 'none')
            with open(dir_cpio, 'rb') as f:
                append_cpio(out, f.read())
            os.unlink(dir_cpio)

        for layer in additional_layers:
            eprint(f"Adding layer {layer}, size: {os.path.getsize(layer)} bytes")
            append_cpio(out, read_layer_as_cpio(layer))

    eprint(f'Size of the uncompressed initial RAM FS {os.path.getsize(cpio_path)} bytes')
    # Same level the kernel build uses for zstd initramfs images.
    subprocess.run([Config.ZSTD, '-19', '-q', '-f', cpio_path, '-o', updated_initramfs_path], check=True)
    eprint(f'Size of the updated initial RAM FS {os.path.getsize(updated_initramfs_path)} bytes')


def process(temp_dir: str, underhill_path: str, kernel_path: str,
            modules_path: str, kernel_metadata_path: str, build_info: str, rootfs_config_path: List[str],
            updated_initramfs_path: str, additional_layers: List[str],
            additional_dirs: List[str], compression: str):
    align = lambda x, boundary: (x + boundary-1) & ~(boundary-1)

    eprint("Building the initial root fs")

    underhill_cpio_gz_file_name = os.path.join(temp_dir, 'underhill.cpio.gz')

    final_underhill_path = os.path.join(temp_dir, 'underhill')
    shutil.copy(underhill_path, final_underhill_path)

    if not build_info:
        build_info = os.path.join(temp_dir, 'openhcl-build-info.json')
        generate_build_info(build_info)

    os.environ["OPENHCL_OPENVMM_PATH"] = final_underhill_path
    os.environ["OPENHCL_KERNEL_PATH"] = kernel_path
    os.environ["OPENHCL_MODULES_PATH"] = modules_path
    os.environ["OPENHCL_KERNEL_METADATA"] = kernel_metadata_path
    os.environ["OPENHCL_BUILD_INFO"] = build_info

    if compression == 'zstd':
        build_zstd_initrd(temp_dir, rootfs_config_path, updated_initramfs_path,
                          additional_layers, additional_dirs)
        return

    stat = create_cpio_from_config(rootfs_config_path, underhill_cpio_gz_file_name, 'gzip')
    eprint(f"The initial root fs size, bytes: {stat}")

    for dir_name in additional_dirs:
        temp_file_name = os.path.join(temp_dir, "add_dir.cpio.gz")
        create_cpio_from_dir(dir_name, temp_file_name, 'gzip')
        append_file(underhill_cpio_gz_file_name, temp_file_name)
        os.unlink(temp_file_name)

    for layer in additional_layers:
        layer_size = os.path.getsize(layer)
        eprint(f"Adding layer {layer}, size: {layer_size} bytes")
        append_file(underhill_cpio_gz_file_name, layer)

    if Config.VERBOSE: subprocess.run(f'binwalk -eM {underhill_cpio_gz_file_name}', shell=True, check=True)
    if Config.VERBOSE: subprocess.run(f'lsinitramfs -l {underhill_cpio_gz_file_name}', shell=True, check=True)

    initgz_file_data = bytes()

    with open(underhill_cpio_gz_file_name, 'rb') as cpiogz_file:
        initgz_file_data = cpiogz_file.read()
        eprint(f'Size of the updated initial RAM FS {len(initgz_file_data)} bytes')

    shutil.move(underhill_cpio_gz_file_name, updated_initramfs_path)

class PackageLayer:
    name = ""
    def __init__(self, name):
        self.name = name


def main():
    import argparse
    import os
    import platform

    # Parsing arguments is a bit laborious here as defaults of some arguments depend on other ones.

    parser = argparse.ArgumentParser(description='Updates the initial RAM FS')

    parser.add_argument('underhill_path', help='Path to underhill')
    parser.add_argument('updated_initramfs_path', help='The path to the updated initramfs')
    parser.add_argument('--arch', default=os.environ.get("UNDERHILL_ARCH"), help='The architecture type (e.g., x86_64, aarch64)')
    parser.add_argument('--package-root', default=os.path.join(get_script_path(), "../.packages", package_id), help='HCL package root containing kernel modules and extra cpio.gz files')
    parser.add_argument('--kernel-modules', required=True, help='Path to kernel modules')
    parser.add_argument('--kernel-metadata', required=True, help='Path to kernel_build_metadata.json')
    parser.add_argument('--build_info', help='Path to the file with build information')
    parser.add_argument('--rootfs-config', action='append', help='Configuration file for the root filesystem')
    parser.add_argument('--layer', action='append', help='Adds a custom layer file.')
    parser.add_argument('--add-dir', action='append', help='Adds a directory.')
    parser.add_argument('--compression', choices=('gzip', 'zstd'), default='gzip',
        help='Initrd compression. zstd needs CONFIG_RD_ZSTD in the OpenHCL kernel.')
    parser.set_defaults(layer=[], add_dir=[], rootfs_config=[])

    args, _  = parser.parse_known_args()

    arch = args.arch
    if arch is None:
        arch = platform.processor()

    if not args.rootfs_config:
        args.rootfs_config = [os.path.join(get_script_path(), "../underhill/rootfs.config")]

    kernel_arch = None
    package_arch = None
    if arch == 'x86_64':
        kernel_arch = 'x64'
        package_arch = 'x64'
        deps_env = "OPENVMM_DEPS_X64"
    elif arch == 'aarch64':
        kernel_arch = 'arm64'
        package_arch = 'aarch64'
        deps_env = "OPENVMM_DEPS_AARCH64"
    else:
        raise Exception(f"Unsupported target arch: {arch}")

    deps=os.environ.get(deps_env)
    if deps is None:
        # Try with replacing OPENVMM_ with HVLITE_.
        deps = os.environ.get(deps_env.replace("OPENVMM_", "HVLITE_"))

    if deps is None:
        deps = os.path.join(get_script_path(), f"../.packages/underhill-deps-private/{package_arch}")
    else:
        deps = os.path.realpath(deps)

    parser.add_argument('--min-interactive', action='append_const', dest='layer', const=os.path.join(deps, "shell.cpio.gz"), help='Add a minimal set of interactive tools for production diagnostics.')
    parser.add_argument('--interactive', action='append_const', dest='layer', const=os.path.join(deps, "dbgrd.cpio.gz"), help='Add interactive tools for development and diagnostics.')

    # Re-parse args to perform actions
    args = parser.parse_args()

    # For the initrd config, we need to know the kernel arch
    os.environ["OPENHCL_KERNEL_ARCH"] = kernel_arch

    for required_tool in Config.REQUIRED_TOOLS:
        if shutil.which(required_tool) is None:
            raise Exception(f"Can't find {required_tool}")
    if args.compression == 'zstd' and shutil.which(Config.ZSTD) is None:
        raise Exception(f"Can't find {Config.ZSTD}, required for --compression zstd")

    underhill_path = os.path.realpath(args.underhill_path)
    build_info = args.build_info
    kernel_path = os.path.realpath(args.package_root)
    modules_path = os.path.realpath(args.kernel_modules)
    kernel_metadata_path = os.path.realpath(args.kernel_metadata)
    updated_initramfs_path = args.updated_initramfs_path

    additional_layers = []
    for layer in args.layer:
        if isinstance(layer, PackageLayer):
            additional_layers.append(os.path.join(args.package_root, f'{layer.name}.cpio.gz'))
        elif os.path.exists(layer):
            additional_layers.append(layer)
        else:
            raise Exception(f"Can't find layer file '{layer}'")

    additional_dirs = args.add_dir

    with tempfile.TemporaryDirectory() as temp_dir:
        process(
            str(temp_dir),
            underhill_path, kernel_path, modules_path, kernel_metadata_path, build_info, args.rootfs_config,
            updated_initramfs_path, additional_layers,
            additional_dirs, args.compression)


if __name__ == '__main__':
    try:
        main()
    except Exception as e:
        eprint(f"Error: {e}")
        exit(-1)
