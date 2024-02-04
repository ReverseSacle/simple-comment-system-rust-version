use std::{
	io::Write,
	net::TcpStream
};

pub fn Response405(stream: &mut TcpStream)
{
	println!("HttpServer::ResponseSelector::Response405() => ...");
	stream.write_all(
		"HTTP/1.0 405 METHOD NOT ALLOWED\r\n\r\n".as_bytes()
	).unwrap();
	stream.flush().unwrap();
	println!("HttpServer::ResponseSelector::Response405() => Done");
}

pub fn Response404(stream: &mut TcpStream)
{
	println!("HttpServer::ResponseSelector::Response404() => ...");
	stream.write_all(
		"HTTP/1.0 404 NOT FOUND\r\n\r\n".as_bytes()
	).unwrap();
	stream.flush().unwrap();
	println!("HttpServer::ResponseSelector::Response404() => Done");
}

pub fn Response403(stream: &mut TcpStream)
{
	println!("HttpServer::ResponseSelector::Response403() => ...");
	stream.write_all(
		"HTTP/1.0 403 FORBIDDEN\r\n\r\n".as_bytes()
	).unwrap();
	stream.flush().unwrap();
	println!("HttpServer::ResponseSelector::Response403() => Done");
}

pub fn Response400(stream: &mut TcpStream)
{
	println!("HttpServer::ResponseSelector::Response400() => ...");
	stream.write_all(
		"HTTP/1.0 400 BAD REQUEST\r\n\r\n".as_bytes()
	).unwrap();
	stream.flush().unwrap();
	println!("HttpServer::ResponseSelector::Response400() => Done");
}
