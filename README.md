# VTable Hook
Simple crate for hooking C++ VTables

# Installing
Add this line to your `Cargo.toml`
```toml
[dependencies]
vtable_hook = { version = "0.1.1" }
```

# Available methods
## Copy
Replacing original VTable with our own copy of that VTable. Available in two implementations:
* Default (or wrapped) - uses lifetimes to prevent undefined behaviour when victim class gets freed. Disables itself on drop.
* Raw - uses raw pointers, undefined behaviour should be prevented by the user.
## Rewrite original VTable
TODO

# Example usage
See [here](./examples/)