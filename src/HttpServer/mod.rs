#![allow(non_snake_case)]
#![allow(dead_code)]
mod RequestType;
mod ResponseSelector;

use std::net::TcpStream;
use RequestType::{ 
	RequestMethodType,
	GetRequestMethodType
};
use ResponseSelector::{ 
	RequestContentParse,
	ResponseSelector, 
	Response4xx::Response405
};

pub fn HttpRequestAccept(mut stream: &mut TcpStream)
{
	println!("HttpServer::HttpRequestAccept() => ...");
	if let Some(request_content) = RequestContentParse(&stream)
	{
		let mut allowed_method = false;

		match GetRequestMethodType(&request_content.req_method)
		{
			RequestMethodType::GET => allowed_method = true,
			RequestMethodType::POST => allowed_method = true,
			_ => {}
		}
		if allowed_method
		{
			ResponseSelector(
				&mut stream,
				&request_content	
			);
		}
		else
		{
			println!("HttpServer::HttpRequestAccept() => NO SUPPORT");
			Response405(&mut stream);
		}
	}
}
