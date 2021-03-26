Validators
-----------
[![Build Status](https://travis-ci.com/marirs/validators-rs.svg?branch=main)](https://travis-ci.com/marirs/validators-rs)

Common validators that can be used across projects.

#### Requirements
- Rust

### Usage
- with all validations
```toml
[dependencies]
validators = { git = "https://github.com/marirs/validators-rs", branch = "main", features = ["validators-all"] }
```
- with crypto validations
```toml
[dependencies]
validators = { git = "https://github.com/marirs/validators-rs", branch = "main", features = ["crypto"] }
```

### Tests
```bash
cargo test --features="validators-all"
```

### Examples
```bash
$ cargo run --example crypto --features="crypto"
warning: unused manifest key: package.author
   Compiling validators v0.1.0 (validators-rs)
    Finished dev [optimized + debuginfo] target(s) in 1.88s
     Running `target/debug/examples/crypto`
1GiWxH6PzSSmbdcK72XfGpqhjSb6nae6h9 => Some("Bitcoin")
qppjlghjlwg6tgxv7ffhvs43rlul0kpp4c0shk4dr6 => Some("Bitcoin Cash")
0xaae47eae4ddd4877e0ae0bc780cfaee3cc3b52cb => Some("Ethereum")
LQ4i7FLNhfCC9GXw682mS1NzvVKbtJAFZq => Some("Litecoin")
D6K2nqqQKycTucCSFSHhpiig4yQ6NPQRf9 => Some("Dodgecoin")
XqLYPDTADW6EYuQmTcEAx81o8EHTKwqTK8 => Some("Dash")
41gYNjXMeXaTmZFVv645A1HRVoA637cXFGbDdLV8Gn5hLvfxfRLKigUTvm2HVZhBzDVPeGpDy71qxASTpRFgepDwLexA8Ti => Some("Monero")
AeHauBkGkHPTxh4PEUhNr7WRgivmcdCRnR => Some("Neo")
rUocf1ixKzTuEe34kmVhRvGqNCofY1NJzV => Some("Ripple")
```

---
License: MIT