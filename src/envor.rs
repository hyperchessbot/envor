/// parse to option of type with default
pub fn env_or_opt<T, K, D>(key: K, default: D) -> Option<T>
where
    T: std::str::FromStr,
    K: core::fmt::Display,
    D: core::fmt::Display,
{
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
where
    T: std::str::FromStr,
    K: core::fmt::Display,
    D: core::fmt::Display,
{
    env_or_opt(key, default).unwrap()
}

/// parse to string with default
pub fn env_string_or<K, D>(key: K, default: D) -> String
where
    K: core::fmt::Display,
    D: core::fmt::Display,
{
    let key = key.to_string();
    let default = default.to_string();

    if let Ok(value) = std::env::var(key) {
        return value;
    }

    default
}

/// env true
pub fn env_true<K>(key: K) -> bool
where
    K: core::fmt::Display,
{
    let key = key.to_string();

    if let Ok(value) = std::env::var(key) {
        return value.to_lowercase() == "true";
    }

    false
}

#[test]
fn test() {
    let flag = env_true("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    assert_eq!(flag, false);

    std::env::set_var(
        "FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE",
        "tRUe",
    );

    let flag = env_true("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    assert_eq!(flag, true);

    std::env::set_var(
        "FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE",
        "tRUeX",
    );

    let flag = env_true("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    assert_eq!(flag, false);

    std::env::set_var(
        "FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE",
        "TRUE",
    );

    let flag = env_true("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    assert_eq!(flag, true);

    std::env::remove_var("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    let flag = env_true("FLAG_ONLY_TRUE_WHEN_DEFINED_AND_ITS_LOWER_CASE_IS_EQUAL_TO_TRUE");

    assert_eq!(flag, false);
}
