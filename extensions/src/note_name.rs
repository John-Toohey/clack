#![deny(missing_docs)]

//! A way for plugins to list custom note names for hosts to display in e.g. a piano roll.

use clack_common::extensions::{Extension, HostExtensionSide, PluginExtensionSide};
use clap_sys::ext::note_name::*;
use std::ffi::CStr;

/// The Plugin-side of the Note Name extension.
#[repr(C)]
pub struct PluginNoteName(clap_plugin_note_name);

unsafe impl Extension for PluginNoteName {
    const IDENTIFIER: &'static CStr = CLAP_EXT_NOTE_NAME;
    type ExtensionSide = PluginExtensionSide;
}

/// The Host-side of the Note Name extension.
#[repr(C)]
pub struct HostNoteName(clap_host_note_name);

unsafe impl Extension for HostNoteName {
    const IDENTIFIER: &'static CStr = CLAP_EXT_NOTE_NAME;
    type ExtensionSide = HostExtensionSide;
}

#[derive(Copy, Clone, Debug)]
/// A Note's name.
pub struct NoteName<'a> {
    /// A user-facing display name for the note.
    pub name: &'a [u8],

    /// The Port this note name applies to, or `-1` if it applies to every key.
    pub port: i16,

    /// The MIDI Channel this note name applies to, or `-1` if it applies to every key.
    pub channel: i16,

    /// The Key this note name applies to, or `-1` if it applies to every key.
    pub key: i16,
}

impl<'a> NoteName<'a> {
    #[cfg(feature = "clack-host")]
    // TODO: make pub?
    unsafe fn from_raw(raw: &'a clap_note_name) -> Self {
        use crate::utils::data_from_array_buf;

        Self {
            name: data_from_array_buf(&raw.name),

            port: raw.port,
            key: raw.key,
            channel: raw.channel,
        }
    }
}

#[cfg(feature = "clack-host")]
mod host;

#[cfg(feature = "clack-host")]
pub use host::*;

#[cfg(feature = "clack-plugin")]
mod plugin;

#[cfg(feature = "clack-plugin")]
pub use plugin::*;
