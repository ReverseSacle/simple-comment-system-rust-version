use std::{
	net::{ TcpListener }
};

pub fn ServerListener(ip_addr: &str,port: u16) 
-> Result<TcpListener,()>
{
	println!("TcpServer::ServerListener() => ...");
	match TcpListener::bind(&format!("{}:{}",ip_addr,port))
	{
		Ok(succeed) => {
			println!("TcpServer::ServerListener() => Done");
			Ok(succeed)
		},
		_ => Err(())
	}
}
