use super::*;
use clack_common::extensions::ExtensionImplementation;
use clack_host::host::PluginHoster;
use clack_host::plugin::PluginMainThreadHandle;
use clack_host::wrapper::HostWrapper;
use clap_sys::host::clap_host;

impl PluginAudioPorts {
    pub fn count(&self, plugin: &PluginMainThreadHandle, is_input: bool) -> u32 {
        unsafe { (self.0.count)(plugin.as_raw(), is_input) }
    }

    pub fn get<'b>(
        &self,
        plugin: &PluginMainThreadHandle,
        index: u32,
        is_input: bool,
        buffer: &'b mut AudioPortInfoBuffer,
        // TODO: handle errors
    ) -> Option<AudioPortInfoData<'b>> {
        let success =
            unsafe { (self.0.get)(plugin.as_raw(), index, is_input, buffer.inner.as_mut_ptr()) };

        if success {
            unsafe { AudioPortInfoData::try_from_raw(buffer.inner.assume_init_ref()) }.ok()
        } else {
            None
        }
    }
}

pub trait HostAudioPortsImplementation {
    fn is_rescan_flag_supported(&self, flag: RescanType) -> bool;
    fn rescan(&mut self, flag: RescanType);
}

impl<'a, H: PluginHoster<'a>> ExtensionImplementation<H> for HostAudioPorts
where
    H: HostAudioPortsImplementation,
{
    const IMPLEMENTATION: &'static Self = &HostAudioPorts(
        clap_host_audio_ports {
            is_rescan_flag_supported: is_rescan_flag_supported::<H>,
            rescan: rescan::<H>,
        },
        PhantomData,
    );
}

unsafe extern "C" fn is_rescan_flag_supported<'a, H: PluginHoster<'a>>(
    host: *const clap_host,
    flag: u32,
) -> bool
where
    H: HostAudioPortsImplementation,
{
    HostWrapper::<H>::handle(host, |host| {
        Ok(host
            .main_thread()
            .as_ref()
            .is_rescan_flag_supported(RescanType::from_bits_truncate(flag)))
    })
    .unwrap_or(false)
}

unsafe extern "C" fn rescan<'a, H: PluginHoster<'a>>(host: *const clap_host, flag: u32)
where
    H: HostAudioPortsImplementation,
{
    HostWrapper::<H>::handle(host, |host| {
        host.main_thread()
            .as_mut()
            .rescan(RescanType::from_bits_truncate(flag));

        Ok(())
    });
}
