# config-better-rs

Make use of directories for configuration / data / caching better and more user-friendly!

This module provides support for the XDG Base Directory specification, and OS-friendly fallbacks for Windows, Mac OS, and Linux if not otherwise specified.

This is a port of the [Python version](https://github.com/kade-robertson/config-better) of the same name.

## Usage

```rust
use config_better::Config;

fn main() {
    let dirs = Config::new("some-app");
    println!("{:?}", dirs);

    // View paths
    println!("{:?}", dirs.cache);
    println!("{:?}", dirs.config);
    println!("{:?}", dirs.data);

    // Create/delete a single directory
    dirs.cache.create();
    dirs.cache.remove();

    // Create/delete all directories
    dirs.create_all();
    dirs.remove_all();
}
```

Any directory `create` and `remove` methods can have an async counterpart provided by the `async` feature.

For further details, refer to installation instructions on [crates.io](https://crates.io/crates/config-better) and docs on [docs.rs](https://docs.rs/config-better).

### MSRV

The MSRV is 1.63.
