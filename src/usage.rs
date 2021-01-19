use envor::envor::*;

fn main(){
	let config_value:usize = env_or("CONFIG_VALUE", 100);

	let flag:bool = env_or("SETTING", false);

	let config_string = env_string_or("CONFIG_STRING", "default string");

	println!("config value = {} , setting = {} , config_string = {}",
		config_value, flag, config_string);
}
