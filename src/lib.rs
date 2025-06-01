#![no_std]

pub mod klyx;

/// Trait that every extension must implement.
pub trait KlyxExtension {
    /// Required memory size (in bytes)
    fn requested_memory_size() -> u32;

    /// Entry point
    fn start();
}

/// Macro to register user extension implementation
#[macro_export]
macro_rules! register_extension {
    ($ty:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn requested_memory_size() -> u32 {
            <$ty as $crate::KlyxExtension>::requested_memory_size()
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn start() {
            <$ty as $crate::KlyxExtension>::start()
        }
    };
}
