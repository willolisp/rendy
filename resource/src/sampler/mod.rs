//! Sampler creation-info and wrappers.

mod cache;
use hal::{device::Device as _, image::SamplerDesc, Backend};
use relevant::Relevant;
use rendy_core::{hal, Device, DeviceId};

pub use crate::sampler::cache::SamplerCache;

/// Generic sampler resource wrapper.
#[derive(Debug)]
pub struct Sampler<B: Backend> {
    device: DeviceId,
    raw: B::Sampler,
    info: SamplerDesc,
    relevant: Relevant,
}

impl<B> Sampler<B>
where
    B: Backend,
{
    /// Create new sampler.
    pub fn create(
        device: &Device<B>,
        info: SamplerDesc,
    ) -> Result<Self, hal::device::AllocationError> {
        // TODO: Check info is valid.
        let raw = unsafe { device.create_sampler(&info) }?;
        Ok(Sampler {
            device: device.id(),
            raw,
            info,
            relevant: Relevant,
        })
    }

    /// Destroy sampler resource.
    pub unsafe fn dispose(self, device: &Device<B>) {
        device.destroy_sampler(self.raw);
        self.relevant.dispose();
    }

    /// Get reference to raw sampler resource.
    pub fn raw(&self) -> &B::Sampler {
        &self.raw
    }

    /// Get mutable reference to raw sampler resource.
    pub unsafe fn raw_mut(&mut self) -> &mut B::Sampler {
        &mut self.raw
    }
}
