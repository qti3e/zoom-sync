//! Utilities for getting system info

use std::sync::LazyLock;

use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Device, Nvml};

static NVML: LazyLock<Option<Nvml>> = LazyLock::new(|| {
    let nvml = Nvml::init().ok();
    if nvml.is_none() {
        eprintln!("warning: nvml not found");
    }
    nvml
});

/// Helper struct to track gpu temperature
pub struct GpuTemp {
    maybe_device: Option<Device<'static>>,
}

impl GpuTemp {
    pub fn new(index: Option<u32>) -> Self {
        let maybe_device = NVML.as_ref().and_then(|nvml| {
            let device = nvml.device_by_index(index.unwrap_or_default()).ok();
            if device.is_none() {
                eprintln!("warning: device not found")
            }
            device
        });

        Self { maybe_device }
    }

    pub fn get_temp(&self) -> Option<u8> {
        self.maybe_device
            .as_ref()
            .and_then(|d| d.temperature(TemperatureSensor::Gpu).ok())
            .map(|v| v as u8)
    }
}
