[![documentation](https://docs.rs/envor/badge.svg)](https://docs.rs/envor) [![Crates.io](https://img.shields.io/crates/v/envor.svg)](https://crates.io/crates/envor) [![Crates.io (recent)](https://img.shields.io/crates/dr/envor)](https://crates.io/crates/envor)

# envor

Get env vars to types with default.

# Usage

```rust
use envor::envor::*;

fn main(){
	let config_value:usize = env_or("CONFIG_VALUE", 100);

	let flag:bool = env_or("SETTING", false);

	let config_string = env_string_or("CONFIG_STRING", "default string");

	println!("config value = {} , setting = {} , config_string = {}",
		config_value, flag, config_string);
}
```

# Advanced

```rust
use envor::envor::*;

fn main(){
	let opt_value_ok:Option<i32>
		= env_or_opt("OPT_VALUE", -50);
		
	let opt_value_illegal:Option<i32>
		= env_or_opt("OPT_VALUE", "illegal default");

	println!("opt_value_ok = {:?} , opt_value_illegal = {:?}",
		opt_value_ok, opt_value_illegal);
}
```

# Logging

```bash
export RUST_LOG=info
# or
export RUST_LOG=debug
```