use wit::*;

// WIT re-exports.
//
// We explicitly enumerate the symbols we want to re-export, as there are some
// that we may want to shadow to provide a cleaner Rust API.
pub use wit::{
    DownloadedFileType, download_file,
    klyx::extension::system::{ToastDuration, show_toast},
};

// Undocumented WIT re-exports.
//
// These are symbols that need to be public for the purposes of implementing
// the extension host, but aren't relevant to extension authors.
#[doc(hidden)]
pub use wit::Guest;

/// A result returned from a Klyx extension.
pub type Result<T, E = String> = core::result::Result<T, E>;

/// A Klyx extension.
pub trait Extension: Send + Sync {
    /// Returns a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized;

    /// This is called when the extension is removed from Klyx.
    /// It can be used to clean up resources or perform any necessary
    /// finalization.
    fn uninstall(&mut self) {
        // Default implementation does nothing.
    }
}

/// Registers the provided type as a Klyx extension.
///
/// The type must implement the [`Extension`] trait.
#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[unsafe(export_name = "init-extension")]
        pub extern "C" fn __init_extension() {
            klyx_extension_api::try_set_current_dir_from_env();

            klyx_extension_api::register_extension(|| {
                Box::new(<$extension_type as klyx_extension_api::Extension>::new())
            });
        }
    };
}

#[doc(hidden)]
#[cfg(target_family = "wasm")]
pub fn try_set_current_dir_from_env() {
    if let Ok(pwd) = std::env::var("PWD") {
        if let Err(e) = std::env::set_current_dir(&pwd) {
            eprintln!("Failed to set current dir to PWD ('{}'): {}", pwd, e);
        }
    } else {
        eprintln!("PWD not set in environment");
    }
}

#[doc(hidden)]
#[cfg(not(target_family = "wasm"))]
pub fn try_set_current_dir_from_env() {
    if let Ok(pwd) = std::env::var("PWD") {
        let _ = std::env::set_current_dir(pwd);
    }
}

#[doc(hidden)]
pub fn register_extension(build_extension: fn() -> Box<dyn Extension>) {
    unsafe { EXTENSION = Some((build_extension)()) }
}

fn extension() -> &'static mut dyn Extension {
    #[expect(static_mut_refs)]
    unsafe {
        EXTENSION.as_deref_mut().unwrap()
    }
}

static mut EXTENSION: Option<Box<dyn Extension>> = None;

#[cfg(target_arch = "wasm32")]
#[unsafe(link_section = "klyx:api-version")]
#[doc(hidden)]
pub static KLYX_API_VERSION: [u8; 6] = *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

mod wit {
    wit_bindgen::generate!({
        skip: ["init-extension"],
        path: "./wit/since_v1.2.1",
    });
}

wit::export!(Component);

struct Component;

impl Guest for Component {
    fn uninstall() {
        extension().uninstall();
    }
}
