#![no_std]

pub mod android;
pub mod ui;

/// Trait that every extension must implement.
pub trait Extension {
    /// Entry point
    fn init();
}

/// Macro to register user extension implementation
#[macro_export]
macro_rules! register_extension {
    ($ty:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn init() {
            <$ty as $crate::Extension>::init()
        }
    };
}
