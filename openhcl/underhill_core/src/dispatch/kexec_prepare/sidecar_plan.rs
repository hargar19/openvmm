// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Plan a fresh incoming sidecar in the reserved region the running sidecar
//! does not use.

use anyhow::Context;
use anyhow::bail;
use anyhow::ensure;
use loader_defs::kexec::MAX_SIDECAR_PLAN_CHUNKS;
use loader_defs::kexec::PAGE_ALIGNMENT;
use loader_defs::kexec::SIDECAR_DT_CHUNKS;
use loader_defs::kexec::SIDECAR_DT_CPU_COUNT;
use loader_defs::kexec::SIDECAR_DT_IMAGE_SIZE;
use loader_defs::kexec::SIDECAR_DT_NODE_NAME;
use loader_defs::kexec::SIDECAR_DT_REGIONS;
use loader_defs::kexec::SIDECAR_PLAN_FLAG_LOGGING;
use loader_defs::kexec::SidecarPlan;
use loader_defs::kexec::SidecarPlanChunk;
use std::ops::Range;
use zerocopy::FromZeros;

const SIDECAR_COMPATIBLE: &[u8] = b"microsoft,openhcl-sidecar";

/// A staged plan and where the post-save record must be written.
pub(crate) struct SidecarStaging {
    pub plan: Box<SidecarPlan>,
    pub record_gpa: u64,
}

struct Property<'a> {
    name: &'a [u8],
    value: &'a [u8],
    offset: usize,
}

struct Node<'a> {
    depth: usize,
    name: &'a [u8],
    properties: Vec<Property<'a>>,
}

impl<'a> Node<'a> {
    fn property(&self, name: &str) -> Option<&Property<'a>> {
        self.properties
            .iter()
            .find(|property| property.name == name.as_bytes())
    }
}

fn be_u32(data: &[u8], offset: usize) -> anyhow::Result<u32> {
    Ok(u32::from_be_bytes(
        data.get(offset..offset + 4)
            .context("truncated device tree")?
            .try_into()?,
    ))
}

fn be_u64s(value: &[u8]) -> anyhow::Result<Vec<u64>> {
    ensure!(
        value.len().is_multiple_of(8),
        "device tree property is not a u64 array"
    );
    Ok(value
        .chunks_exact(8)
        .map(|v| u64::from_be_bytes(v.try_into().unwrap()))
        .collect())
}

fn c_string(data: &[u8], offset: usize) -> anyhow::Result<&[u8]> {
    let data = data.get(offset..).context("string offset out of bounds")?;
    let len = data
        .iter()
        .position(|&b| b == 0)
        .context("unterminated device tree string")?;
    Ok(&data[..len])
}

fn block(fdt: &[u8], offset: u32, size: u32) -> anyhow::Result<(usize, &[u8])> {
    let start = offset as usize;
    let end = start
        .checked_add(size as usize)
        .context("device tree block overflow")?;
    Ok((
        start,
        fdt.get(start..end)
            .context("device tree block out of bounds")?,
    ))
}

/// Flatten the structure block, recording each property's absolute offset.
fn parse_nodes(fdt: &[u8]) -> anyhow::Result<Vec<Node<'_>>> {
    ensure!(be_u32(fdt, 0)? == 0xd00dfeed, "invalid device tree magic");
    let total = be_u32(fdt, 4)? as usize;
    ensure!(total <= fdt.len(), "truncated device tree");
    let fdt = &fdt[..total];
    let (struct_base, structure) = block(fdt, be_u32(fdt, 8)?, be_u32(fdt, 36)?)?;
    let (_, strings) = block(fdt, be_u32(fdt, 12)?, be_u32(fdt, 32)?)?;
    let mut nodes: Vec<Node<'_>> = Vec::new();
    let mut stack = Vec::new();
    let mut cursor = 0;
    loop {
        let token = be_u32(structure, cursor)?;
        cursor += 4;
        match token {
            1 => {
                let name = c_string(structure, cursor)?;
                cursor = (cursor + name.len() + 1).next_multiple_of(4);
                stack.push(nodes.len());
                nodes.push(Node {
                    depth: stack.len() - 1,
                    name,
                    properties: Vec::new(),
                });
            }
            2 => {
                stack.pop().context("unbalanced device tree")?;
            }
            3 => {
                let len = be_u32(structure, cursor)? as usize;
                let name = c_string(strings, be_u32(structure, cursor + 4)? as usize)?;
                let start = cursor + 8;
                let value = structure
                    .get(start..start + len)
                    .context("truncated device tree property")?;
                let node = *stack.last().context("property outside node")?;
                nodes[node].properties.push(Property {
                    name,
                    value,
                    offset: struct_base + start,
                });
                cursor = (start + len).next_multiple_of(4);
            }
            4 => {}
            9 => break,
            token => bail!("invalid device tree token {token:#x}"),
        }
    }
    ensure!(stack.is_empty(), "unbalanced device tree");
    Ok(nodes)
}

fn region_index(regions: &[Range<u64>; 2], address: u64) -> Option<usize> {
    regions.iter().position(|region| region.contains(&address))
}

/// Build a plan from the device tree that kexec will carry to the successor,
/// targeting the reserved region whose memory no running VP can reference.
///
/// Returns `None` if the running boot did not reserve kexec sidecar regions.
pub(crate) fn plan(fdt: &[u8], enable_logging: bool) -> anyhow::Result<Option<SidecarStaging>> {
    let nodes = parse_nodes(fdt)?;
    let Some(kexec) = nodes
        .iter()
        .find(|node| node.depth == 1 && node.name == SIDECAR_DT_NODE_NAME.as_bytes())
    else {
        return Ok(None);
    };
    let property = |name| {
        kexec
            .property(name)
            .with_context(|| format!("kexec sidecar node lacks {name}"))
    };
    let cpu_count = u32::from_be_bytes(
        property(SIDECAR_DT_CPU_COUNT)?
            .value
            .try_into()
            .context("invalid cpu-count")?,
    );
    let image_size = match be_u64s(property(SIDECAR_DT_IMAGE_SIZE)?.value)?[..] {
        [size] => size,
        _ => bail!("invalid image-size"),
    };
    let regions = match be_u64s(property(SIDECAR_DT_REGIONS)?.value)?[..] {
        [a, a_len, b, b_len] => [
            a..a.checked_add(a_len).context("region overflow")?,
            b..b.checked_add(b_len).context("region overflow")?,
        ],
        _ => bail!("invalid regions"),
    };
    let chunks = be_u64s(property(SIDECAR_DT_CHUNKS)?.value)?;
    ensure!(
        chunks.len() % 4 == 0 && chunks.len() / 4 <= MAX_SIDECAR_PLAN_CHUNKS,
        "invalid chunks"
    );

    let mut regs = Vec::new();
    for node in &nodes {
        let Some(compatible) = node.property("compatible") else {
            continue;
        };
        if !compatible
            .value
            .split(|&b| b == 0)
            .any(|value| value == SIDECAR_COMPATIBLE)
        {
            continue;
        }
        let reg = node.property("reg").context("sidecar node lacks reg")?;
        let values = be_u64s(reg.value)?;
        ensure!(values.len() == 4, "sidecar reg is not four cells");
        regs.push((reg.offset, values[0]));
    }
    ensure!(
        regs.len() == chunks.len() / 4,
        "{} sidecar nodes for {} kexec chunks",
        regs.len(),
        chunks.len() / 4
    );

    // The running sidecar is either the boot-shim one or in one of the regions.
    let active = regs
        .first()
        .and_then(|&(_, ctrl)| region_index(&regions, ctrl));
    ensure!(
        regs.iter()
            .all(|&(_, ctrl)| region_index(&regions, ctrl) == active),
        "sidecar nodes span multiple regions"
    );
    let target = &regions[if active == Some(0) { 1 } else { 0 }];

    let mut plan = SidecarPlan::new_box_zeroed().map_err(|_| anyhow::anyhow!("out of memory"))?;
    plan.magic = SidecarPlan::MAGIC;
    plan.version = SidecarPlan::VERSION;
    plan.flags = if enable_logging {
        SIDECAR_PLAN_FLAG_LOGGING
    } else {
        0
    };
    plan.region_base = target.start;
    plan.region_size = target.end - target.start;
    plan.image_size = image_size;
    plan.record_offset = image_size
        .checked_sub(PAGE_ALIGNMENT)
        .context("image-size too small")?;
    plan.cpu_count = cpu_count;
    plan.chunk_count = regs.len() as u32;
    // Room for an alternating CPU list; wider lists fall back to Linux-only.
    plan.boot_cpus_width = cpu_count
        .saturating_mul(2)
        .saturating_add(16)
        .clamp(64, 1024);
    for ((chunk, values), &(reg_offset, _)) in plan
        .chunks
        .iter_mut()
        .zip(chunks.chunks_exact(4))
        .zip(&regs)
    {
        *chunk = SidecarPlanChunk {
            base_vp: values[0].try_into().context("invalid base VP")?,
            vp_count: values[1].try_into().context("invalid VP count")?,
            arena_offset: values[2],
            arena_size: values[3],
            dt_reg_offset: reg_offset.try_into().context("device tree too large")?,
            reserved: 0,
        };
    }
    ensure!(plan.validate(), "invalid kexec sidecar layout");
    let record_gpa = plan.region_base + plan.record_offset;
    Ok(Some(SidecarStaging { plan, record_gpa }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use loader_defs::kexec::SIDECAR_REGION_IMAGE_SIZE;
    use test_with_tracing::test;

    const REGIONS: [u64; 4] = [0x4000_0000, 0x40_0000, 0x4040_0000, 0x40_0000];

    fn fdt(ctrl: &[u64], chunk_count: usize) -> Vec<u8> {
        let mut buf = vec![0; 0x4000];
        let mut builder = fdt::builder::Builder::new(fdt::builder::BuilderConfig {
            blob_buffer: &mut buf,
            string_table_cap: 256,
            memory_reservations: &[],
        })
        .unwrap();
        let p_compatible = builder.add_string("compatible").unwrap();
        let p_reg = builder.add_string("reg").unwrap();
        let p_cpu_count = builder.add_string(SIDECAR_DT_CPU_COUNT).unwrap();
        let p_image_size = builder.add_string(SIDECAR_DT_IMAGE_SIZE).unwrap();
        let p_regions = builder.add_string(SIDECAR_DT_REGIONS).unwrap();
        let p_chunks = builder.add_string(SIDECAR_DT_CHUNKS).unwrap();
        let mut bus = builder
            .start_node("")
            .unwrap()
            .start_node("bus")
            .unwrap()
            .add_str(p_compatible, "simple-bus")
            .unwrap();
        for &ctrl in ctrl {
            bus = bus
                .start_node(&format!("sidecar@{ctrl:x}"))
                .unwrap()
                .add_str(p_compatible, "microsoft,openhcl-sidecar")
                .unwrap()
                .add_u64_array(p_reg, &[ctrl, 0x1000, ctrl + 0x1000, 0x40000])
                .unwrap()
                .end_node()
                .unwrap();
        }
        let chunks = [
            0,
            32,
            SIDECAR_REGION_IMAGE_SIZE,
            0x10_0000,
            32,
            32,
            SIDECAR_REGION_IMAGE_SIZE + 0x10_0000,
            0x10_0000,
        ];
        let root = bus
            .end_node()
            .unwrap()
            .start_node(SIDECAR_DT_NODE_NAME)
            .unwrap()
            .add_u32(p_cpu_count, 64)
            .unwrap()
            .add_u64(p_image_size, SIDECAR_REGION_IMAGE_SIZE)
            .unwrap()
            .add_u64_array(p_regions, &REGIONS)
            .unwrap()
            .add_u64_array(p_chunks, &chunks[..4 * chunk_count])
            .unwrap()
            .end_node()
            .unwrap();
        let len = root.end_node().unwrap().build(0).unwrap();
        buf.truncate(len);
        buf
    }

    #[test]
    fn plan_alternates_regions_and_locates_reg_cells() {
        let arena = |region: usize, chunk: u64| {
            REGIONS[region * 2] + SIDECAR_REGION_IMAGE_SIZE + chunk * 0x10_0000
        };
        for (ctrl, target) in [
            ([0x1000_0000, 0x1010_0000], 0),
            ([arena(0, 0), arena(0, 1)], 1),
            ([arena(1, 0), arena(1, 1)], 0),
        ] {
            let blob = fdt(&ctrl, 2);
            let staging = plan(&blob, true).unwrap().unwrap();
            let plan = &staging.plan;
            assert_eq!(plan.region_base, REGIONS[target * 2]);
            assert_eq!(plan.flags, SIDECAR_PLAN_FLAG_LOGGING);
            assert_eq!(plan.boot_cpus_width, 144);
            assert_eq!(
                staging.record_gpa,
                plan.region_base + SIDECAR_REGION_IMAGE_SIZE - PAGE_ALIGNMENT
            );
            for (chunk, ctrl) in plan.chunks().iter().zip(ctrl) {
                let offset = chunk.dt_reg_offset as usize;
                assert_eq!(blob[offset..offset + 8], ctrl.to_be_bytes());
            }
            assert_eq!(plan.chunks()[1].base_vp, 32);
        }
    }

    #[test]
    fn plan_rejects_inconsistent_device_tree() {
        assert!(plan(&fdt(&[0x1000_0000], 2), false).is_err());
        let arena = REGIONS[0] + SIDECAR_REGION_IMAGE_SIZE;
        assert!(plan(&fdt(&[arena, 0x1000_0000], 2), false).is_err());
        let mut blob = fdt(&[0x1000_0000, 0x1010_0000], 2);
        blob[0] = 0;
        assert!(plan(&blob, false).is_err());
    }

    #[test]
    fn plan_absent_without_kexec_node() {
        let mut buf = vec![0; 0x1000];
        let builder = fdt::builder::Builder::new(fdt::builder::BuilderConfig {
            blob_buffer: &mut buf,
            string_table_cap: 64,
            memory_reservations: &[],
        })
        .unwrap();
        let len = builder
            .start_node("")
            .unwrap()
            .end_node()
            .unwrap()
            .build(0)
            .unwrap();
        assert!(plan(&buf[..len], false).unwrap().is_none());
    }
}
