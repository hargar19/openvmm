// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Definitions for Microsoft Hypervisor reference time.

use inspect::Inspect;
use std::time::Duration;

/// A hypervisor reference time value. This is a 64-bit value that starts at 0
/// when the VM boots (typically) and measures elapsed time in 100ns units.
///
/// It may stop while the VM is paused.
#[derive(Copy, Clone, Debug, Inspect)]
#[inspect(transparent(hex))]
pub struct ReferenceTime(u64);

impl ReferenceTime {
    /// Wraps a reference time value.
    pub fn new(value_100ns: u64) -> Self {
        Self(value_100ns)
    }

    /// Returns the reference time in 100ns units.
    pub fn as_100ns(&self) -> u64 {
        self.0
    }

    /// Computes the change in reference time since `start`.
    ///
    /// Returns `None` if `start` is after `self`.
    pub fn since(&self, start: ReferenceTime) -> Option<Duration> {
        let diff_100ns = self.0.wrapping_sub(start.0);
        if (diff_100ns as i64) < 0 {
            return None;
        }
        // Can't just use from_nanos since that could overflow.
        let count_per_sec = 10 * 1000 * 1000;
        Some(Duration::new(
            diff_100ns / count_per_sec,
            (diff_100ns % count_per_sec) as u32 * 100,
        ))
    }
}

#[derive(Default)]
pub(crate) struct ServicingTimeline {
    pub method: &'static str,
    checkpoints: Vec<(&'static str, Option<u64>)>,
}

impl ServicingTimeline {
    pub fn record(&mut self, phase: &'static str, reference_time: u64) {
        self.record_optional(phase, Some(reference_time));
    }

    pub fn record_optional(&mut self, phase: &'static str, reference_time: Option<u64>) {
        self.checkpoints.push((phase, reference_time));
    }

    pub fn record_boot(
        &mut self,
        kexec: bool,
        boot_times: Option<bootloader_fdt_parser::BootTimes>,
        worker_entry: Option<u64>,
    ) {
        self.method = if kexec { "kexec" } else { "host" };
        if kexec {
            self.record_optional("handoff_kernel_init", worker_entry);
        } else {
            self.record_optional("handoff", boot_times.and_then(|times| times.start));
            self.record_optional("bootloader", boot_times.and_then(|times| times.end));
            self.record_optional("kernel_init", worker_entry);
        }
    }

    pub fn durations(&self) -> Option<Vec<(&'static str, Duration)>> {
        self.checkpoints
            .windows(2)
            .map(|pair| {
                let (_, start) = pair[0];
                let (phase, end) = pair[1];
                Some((
                    phase,
                    ReferenceTime::new(end?).since(ReferenceTime::new(start?))?,
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_with_tracing::test;

    #[test]
    fn servicing_timeline_accounts_for_entire_blackout() {
        let mut timeline = ServicingTimeline::default();
        for (phase, time) in [
            ("stop_begin", 100),
            ("stop", 130),
            ("save", 170),
            ("handoff", 900),
            ("boot", 1500),
            ("restore", 1800),
            ("start", 1900),
        ] {
            timeline.record(phase, time);
        }
        let durations = timeline.durations().unwrap();
        assert_eq!(durations.len(), 6);
        assert_eq!(durations[2], ("handoff", Duration::from_nanos(73000)));
        assert_eq!(
            durations
                .iter()
                .map(|(_, duration)| *duration)
                .sum::<Duration>(),
            ReferenceTime::new(1900)
                .since(ReferenceTime::new(100))
                .unwrap()
        );
    }

    #[test]
    fn servicing_timeline_rejects_backward_clock() {
        let mut timeline = ServicingTimeline::default();
        timeline.record("stop_begin", 100);
        timeline.record("boot", 99);
        assert!(timeline.durations().is_none());
    }

    #[test]
    fn servicing_timeline_groups_missing_checkpoints() {
        let mut timeline = ServicingTimeline::default();
        timeline.record("stop_begin", 100);
        timeline.record("pre_boot", 900);
        timeline.record("start", 1900);
        assert_eq!(
            timeline.durations().unwrap(),
            vec![
                ("pre_boot", Duration::from_nanos(80000)),
                ("start", Duration::from_nanos(100000)),
            ]
        );
    }

    #[test]
    fn servicing_timeline_does_not_invent_missing_times() {
        let mut timeline = ServicingTimeline::default();
        timeline.record("stop_begin", 100);
        timeline.record_optional("boot", None);
        timeline.record("start", 1900);
        assert!(timeline.durations().is_none());
    }

    #[test]
    fn servicing_timeline_kexec_ignores_old_bootloader_times() {
        let old_boot = bootloader_fdt_parser::BootTimes {
            start: Some(10),
            end: Some(20),
            sidecar_start: None,
            sidecar_end: None,
        };
        for boot_times in [None, Some(old_boot)] {
            let mut timeline = ServicingTimeline::default();
            timeline.record("flush", 100);
            timeline.record_boot(true, boot_times, Some(900));
            assert_eq!(timeline.method, "kexec");
            assert_eq!(
                timeline.durations().unwrap(),
                vec![("handoff_kernel_init", Duration::from_nanos(80000))]
            );
        }
    }

    #[test]
    fn servicing_timeline_host_boot_partitions_same_interval() {
        let mut timeline = ServicingTimeline::default();
        timeline.record("flush", 100);
        timeline.record_boot(
            false,
            Some(bootloader_fdt_parser::BootTimes {
                start: Some(200),
                end: Some(300),
                sidecar_start: None,
                sidecar_end: None,
            }),
            Some(900),
        );
        assert_eq!(timeline.method, "host");
        assert_eq!(
            timeline.durations().unwrap(),
            vec![
                ("handoff", Duration::from_nanos(10000)),
                ("bootloader", Duration::from_nanos(10000)),
                ("kernel_init", Duration::from_nanos(60000)),
            ]
        );
    }
}
