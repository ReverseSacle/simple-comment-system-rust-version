use std::{
    io::Write,
    net::TcpStream
};

pub fn Response500(stream: &mut TcpStream)
{
    println!("HttpServer::ResponseSelector::Response500() => ...");
    stream.write_all(
        "HTTP/1.0 500 INTERNAL SERVER ERROR\r\n\r\n".as_bytes()
    ).unwrap();
    stream.flush().unwrap();
    println!("HttpServer::ResponseSelector::Response500() => Done");
}
