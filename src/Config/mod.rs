use std::{
	fs::File,
	io::Read
};
use super::DatabaseServer::DatabaseTable;
use serde::{ Deserialize,Serialize };
use serde_json::from_str;
use lazy_static::lazy_static;

#[derive(Debug,Deserialize,Serialize)]
pub struct LogConfig
{
	pub enable: String,
	pub path: String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ConfigParame
{
	pub server_host: String,
	pub root_dir: String,
	pub log_config: LogConfig,
	pub database_config: DatabaseTable
}

impl ConfigParame
{
	fn Parse() -> Option<ConfigParame>
	{
		println!("ConfigParame::Parse() => ...");
		let file = File::open("./src/Config/_config.json");

		if !file.is_ok(){
			println!("ConfigParame::Parse() => File::open() fail");
		}
		else
		{
			let mut file = file.unwrap();
			let mut buf = String::new();

			if let Ok(_) = file.read_to_string(&mut buf)
			{
				let config: ConfigParame = from_str(&buf).unwrap(); 

				println!("ConfigParame::Parse() => Done");
				return Some(config);
			}
		}
		None
	}
}

lazy_static! {
	pub static ref CONFIG: Option<ConfigParame> = ConfigParame::Parse(); 
}
