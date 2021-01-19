/// parse to option of type with default
pub fn env_or_opt<T, K, D>(key: K, default: D) -> Option<T>
where T: std::str::FromStr, K: core::fmt::Display, D: core::fmt::Display {
	let key = key.to_string();	
	let default = default.to_string();

	let base = std::env::var(key).ok();

	if base.is_some() {
		let parsed = base.unwrap().parse::<T>().ok();

		if parsed.is_some() {
			return parsed;
		}
	}

	default.parse::<T>().ok()
}

/// parse to type with default
pub fn env_or<T, K, D>(key: K, default: D) -> T
where T: std::str::FromStr, K: core::fmt::Display, D: core::fmt::Display {
	env_or_opt(key, default).unwrap()
}

/// parse to string with default
pub fn env_string_or<K, D>(key: K, default: D) -> String
where K: core::fmt::Display, D: core::fmt::Display {
	let key = key.to_string();	
	let default = default.to_string();

	if let Ok(value) = std::env::var(key) {
		return value
	}

	default
}
