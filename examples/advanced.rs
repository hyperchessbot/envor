use envor::envor::*;

fn main() {
    let opt_value_ok: Option<i32> = env_or_opt("OPT_VALUE", -50);

    let opt_value_illegal: Option<i32> = env_or_opt("OPT_VALUE", "illegal default");

    println!(
        "opt_value_ok = {:?} , opt_value_illegal = {:?}",
        opt_value_ok, opt_value_illegal
    );
}
