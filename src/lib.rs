#![no_std]

pub mod android;
pub mod ui;

/// Trait that every extension must implement.
pub trait Extension {
    /// Entry point
    fn start();
}

/// Macro to register user extension implementation
#[macro_export]
macro_rules! register_extension {
    ($ty:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn start() {
            <$ty as $crate::Extension>::start()
        }
    };
}
