use envor::envor::*;

fn main(){
	let config_value:usize = env_or("CONFIG_VALUE", 100);

	let flag:bool = env_or("SETTING", false);

	println!("config value = {} , setting = {}", config_value, flag);
}
