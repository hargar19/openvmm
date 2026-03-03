#!/bin/sh
# Fast kexec exec — used at servicing time when the kernel has already been
# pre-loaded by kexec_prepare.sh.  This script only calls `kexec -e` which
# triggers the immediate jump to the staged kernel.
set -eu

KEXEC_BIN="/sbin/kexec"
[ -x "$KEXEC_BIN" ] || { echo "[KEXEC-EXEC] ERROR: $KEXEC_BIN not found" >&2; exit 1; }

echo "[KEXEC-EXEC] Executing kexec (pre-loaded kernel)"
exec "$KEXEC_BIN" -e
