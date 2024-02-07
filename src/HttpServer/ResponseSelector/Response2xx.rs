use std::{
	io::Write,
	net::TcpStream
};

pub fn Response204(stream: &mut TcpStream)
{
	println!(
		"HttpServer::ResponseSelector::Response204() => ..."
	);
	stream.write_all("HTTP/1.0 204 NO CONTENT\r\n\r\n".as_bytes()).unwrap();
	stream.flush().unwrap();
	println!(
		"HttpServer::ResponseSelector::Response204() => Done"
	);
}

pub fn Response201(stream: &mut TcpStream)
{
	println!(
		"HttpServer::ResponseSelector::Response201() => ..."
	);
	stream.write_all("HTTP/1.0 201 CREATED\r\n\r\n".as_bytes()).unwrap();
	stream.flush().unwrap();
	println!(
		"HttpServer::ResponseSelector::Response201() => Done"
	);
}

pub fn Response200(stream: &mut TcpStream,file_type: &str,
file_content: &Vec<u8>)
{
	println!(
		"HttpServer::ResponseSelector::Response200() => ..."
	);
	let mut buf = String::new();
	let mut binary_buf = Vec::new();

	buf.push_str("HTTP/1.0 200 OK\r\n");
	buf.push_str("Connection: Close\r\n");
	buf.push_str(&format!("Content-Type: {}\r\n",file_type));
	buf.push_str(&format!("Content-Length: {}\r\n\r\n",file_content.len()));

	binary_buf.extend(buf.as_bytes());
	binary_buf.extend(file_content);

	stream.write_all(&binary_buf).unwrap();
	stream.flush().unwrap();
	println!(
		"HttpServer::ResponseSelector::Response200() => Done"
	);
}
