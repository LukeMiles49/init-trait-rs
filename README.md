# init_trait

[Crate](https://crates.io/crates/init_trait)

[Documentation](https://docs.rs/init_trait)

[Repository](https://github.com/LukeMiles49/init-trait-rs)

[Changelog](https://github.com/LukeMiles49/init-trait-rs/blob/master/CHANGELOG.md)

A small helper trait to simplify the initialisation of 'indexable' data structures.

```rust
use init_trait::Init;

struct House { number: usize }

// [T; N]: Init<T, usize>
let road = <[House; 3]>::init(|i| House { number: i + 1 });

assert_eq!(road[0].number, 1);
assert_eq!(road[1].number, 2);
assert_eq!(road[2].number, 3);
```

To use this, add it as a dependency to your Cargo.toml:
```toml
[dependencies]
init_trait = "0.2.0"
```
