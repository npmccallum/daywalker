# daywalker

[![Crates.io](https://img.shields.io/crates/v/daywalker.svg)](https://crates.io/crates/daywalker)
[![Documentation](https://docs.rs/daywalker/badge.svg)](https://docs.rs/daywalker)
[![Build Status](https://github.com/npmccallum/daywalker/workflows/CI/badge.svg)](https://github.com/npmccallum/daywalker/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.63+](https://img.shields.io/badge/rust-1.56+-orange.svg)](https://www.rust-lang.org)

Write nightly-conditional code once. Run on both nightly and stable Rust.

```rust
#![cfg_attr(feature = "nightly", feature(const_trait_impl))]

daywalker::roam! {
    pub ++[const] trait Name {
        fn name(&self) -> &'static str;
    }

    impl ++[const] Name for () {
        fn name(&self) -> &'static str {
            ++["nightly"] --["stable"]
        }
    }
}

fn main() {
    println!("Hello, {}!", ().name());
}
```

Perfect for **library authors** - write once, let users choose between nightly
features or stable compatibility.

## How it works

1. Use the function-like proc-macro: `roam! { ... }`.
2. Inside the proc-macro, use the conditional operators ("bitemarks"):

- **`++[...]`**: Emit code only when `nightly` feature is enabled
- **`--[...]`**: Emit code only when `nightly` feature is disabled

## Features

- **Zero-cost** - Simple conditional inclusion, no runtime overhead
- **Forward compatible** - Easy migration when nightly features stabilize
- **Lightweight** - Single proc-macro, no dependencies
- **Flexible** - Works with any nightly-specific syntax or tokens
- **Granular control** - Conditionally include any code block or token

## For Library Authors

Write nightly-optional code that work for everyone! First, expose the choice to
your library users:

```toml
# Cargo.toml
[package]
name = "cool-lib"
version = "1.0.0"

[dependencies]
daywalker = "1.0"

[features]
nightly = ["daywalker/nightly"]
```

Then, define and implement conditional code using the nightly syntax with prefix
operators:

```rust
// src/lib.rs
#![cfg_attr(feature = "nightly", feature(const_trait_impl))]

daywalker::roam! {
    pub ++[const] trait Compute {
        fn compute(&self) -> u32;
    }

    impl ++[const] Compute for u32 {
        fn compute(&self) -> u32 { *self * 2 }
    }
}
```

## For Library Users

### Stable

If you want to run on stable rust, use the library like normal. First, add the
dependency:

```toml
# Cargo.toml
[dependencies]
cool-lib = "1.0"
```

Then use the dependency:

```rust
// src/main.rs
use cool_lib::Compute;
let value: u32 = 42u32.compute(); // ✅ Runtime
```

That's it!

### Nightly

On the other hand, if you want that sweet nightly functionality and are willing
to accept the requirement to compile only on nightly, then just use the
`nightly` feature:

```toml
# Cargo.toml
[dependencies]
cool-lib = { version = "1.0", features = ["nightly"] }
```

Look, you get nightly features!

```rust
// src/main.rs
#![feature(const_trait_impl)]
use cool_lib::Compute;
const VALUE: u32 = 42u32.compute(); // ✅ Compile-time
```
