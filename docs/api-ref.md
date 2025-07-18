## API Reference

### Core Traits

#### `Extension`
The main trait that all extensions must implement.

```rust
pub trait Extension {
    fn init();
}
```

**Methods:**
- `init()` - Entry point for your extension. Called when the extension is loaded.

### Macros

#### `register_extension!`
Registers your extension implementation with the Klyx runtime.

```rust
register_extension!(MyExtension);
```

**Parameters:**
- `$ty` - Your extension type that implements the `Extension` trait

### Android Module

Provides Android-specific functionality.

#### `show_toast(msg: &str)`
Displays a toast notification on Android devices.

```rust
use klyx_extension_api::android::show_toast;

show_toast("Hello, Android!");
```

**Parameters:**
- `msg` - The message to display in the toast

### UI Module

Currently available for future UI-related functionality.
