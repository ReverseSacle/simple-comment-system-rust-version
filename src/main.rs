extern crate httpserver;
use httpserver::{
	TcpServer,
	HttpServer
};

fn main()
{
	println!("main() => ...");
	if let Ok(listener) = TcpServer::ServerListener("0.0.0.0",80)
	{
		for stream in listener.incoming()
		{
			println!("main() => Waiting incoming");
			if let Ok(mut user) = stream
			{
				HttpServer::HttpRequestAccept(&mut user);	
				println!();
			}
		}
	} else { println!("TcpServer::ServerListener() fail"); }
}
