mod Response2xx;
pub mod Response4xx;
mod Response5xx;

use std::{
	fs::File,
	net::TcpStream,
	io::{ Read,BufReader,BufRead }
};

use super::RequestType::{ 
	self,
	RequestPathType,
	RequestFileType 
};
use super::super::{
	Config::CONFIG,
	DatabaseServer::{
		DatabaseTable,
		Record
	}
};

#[derive(Debug)]
pub struct RequestContent
{
	pub req_method: String,
	pub req_url: String,
	pub req_body_len: String,
	pub req_body: String
}

fn string_to_usize(num_str: &str) -> usize
{
	let mut new_size: usize = 0;

	for c in num_str.chars()
	{
		if ' ' == c{ continue; }
		new_size = new_size * 10 + (c as u8 - 48) as usize;
	}
	new_size
}

fn UrlFormat(url_buf: &str) -> String
{
	println!("HttpServer::ResponseSelector::UrlFormat() => ...");
	let mut format_url = String::new();
	let mut buf_iter = url_buf.chars();
	let mut buf_ptr = buf_iter.next();

	while buf_ptr.is_some()
	{
		while buf_ptr.is_some() && 
		(Some('/') != buf_ptr && Some('\\') != buf_ptr)
		{
			format_url.push(buf_ptr.unwrap());
			buf_ptr = buf_iter.next();
		}
		format_url.push('/');

		while buf_ptr.is_some() && 
		(Some('/') == buf_ptr && Some('\\') == buf_ptr)
		{
			format_url.push(buf_ptr.unwrap());
			buf_ptr = buf_iter.next();
		}
		if buf_ptr.is_some(){ buf_ptr = buf_iter.next(); }
	}
	format_url.pop();

	println!("HttpServer::ResponseSelector::UrlFormat() => \n{}",&format_url);
	format_url	
}

fn StartWith(buf: &str,prefix: &str) -> String
{
	let buf_iter = buf.to_uppercase();

	let mut buf_iter = buf_iter.chars();
	let mut prefix_iter = prefix.chars();
	let mut buf_ptr = buf_iter.next();
	let mut prefix_ptr = prefix_iter.next();

	while prefix_ptr.is_some()
	{
		if buf_ptr.is_none() || buf_ptr != prefix_ptr{
			return String::from("0");
		}

		buf_ptr = buf_iter.next();
		prefix_ptr = prefix_iter.next();
	}

	let mut length = String::new();

	buf_ptr = buf_iter.next();
	while buf_ptr.is_some()
	{
		if let Some('0') | Some('1') | Some('2') | Some('3') |
			Some('4') | Some('5') | Some('6') | Some('7') | 
			Some('8') | Some('9') = buf_ptr{
			length.push(buf_ptr.unwrap());
		}
		buf_ptr = buf_iter.next();
	}
	if length.is_empty(){ length.push('0'); }

	println!("{}<=>{} length: {}",&buf,&prefix,&length);
	length
}

pub fn RequestContentParse(mut stream: &TcpStream) 
-> Option<RequestContent>
{
	println!("HttpServer::ResponseSelector::RequestContentParse() => ...");
	let mut buf = String::new();
	let mut reader = BufReader::new(&mut stream);

	if !reader.read_line(&mut buf).is_ok(){ return None; }
	println!(
		"HttpServer::ResponseSelector::RequestContentParse() => \n{}",
		&buf
	);

	let mut req_content = RequestContent{
		req_method: String::new(),
		req_url: String::new(),
		req_body_len: String::new(),
		req_body: String::new()
	};
	let mut switch_ptr = 0;

	for c in buf.chars()
	{
		if ' ' == c 
		{ 
			switch_ptr += 1;
			continue;
		}
		match switch_ptr
		{
			0 => req_content.req_method.push(c),
			1 => req_content.req_url.push(c),
			_ => break
		}
	}

	while buf.len() > 2
	{
		buf.clear();
		reader.read_line(&mut buf).unwrap();
		println!(
			"HttpServer::ResponseSelector::RequestContentParse() => \n{}",
			&buf
		);

		let body_len = StartWith(&buf,"CONTENT-LENGTH");
		if "0" != body_len{
			req_content.req_body_len.push_str(&body_len);
		} 
	}

	if buf.is_empty()
	{ 	
		req_content.req_body_len.clear();
		req_content.req_body_len.push('0');

		println!(
			"HttpServer::ResponseSelector::RequestContentParse() => \
			Done EMPTY BODY"
		);
		return Some(req_content);
	}
	else
	{
		let size = string_to_usize(&req_content.req_body_len);

		if 0 != size
		{
			let mut buf = vec![0;size];

			if reader.read_exact(&mut buf).is_ok()
			{
				if let Ok(body_str) = String::from_utf8(buf){
					req_content.req_body = body_str;
				}
			}
			else
			{
				println!(
					"HttpServer::ResponseSelector::RequestContentParse() => \
					read_exact fail"
				);
			}
		} 
		println!(
			"HttpServer::ResponseSelector::RequestContentParse() => \
			body size {}\n{}",&size,&req_content.req_body
		);
	}
	println!(
		"HttpServer::ResponseSelector::RequestContentParse() => Done"
	);
	Some(req_content)
}

fn RequestBodyTackle(body_buf: &str) -> Option<Record>
{
	println!(
		"HttpServer::ResponseSelector::RequestBodyTackle() => ..."
	);
	if 0 == body_buf.len(){ return None; }

	let mut record = Record{
		id: String::new(),
		parent_id: None,
		response_id: None,
		nick_name: String::new(),
		mail: String::new(),
		mail_md5: String::new(),
		create_at: String::new(),
		comment: String::new()
	};
		
	let mut body_buf_iter = body_buf.chars();
	let mut body_buf_ptr = body_buf_iter.next();
	let mut member_ptr = 0;
	let mut parent_id = String::new();
	let mut response_id = String::new();

	while body_buf_ptr.is_some()
	{
		if Some(':') == body_buf_ptr && 7 != member_ptr{
			member_ptr += 1;
		} 
		else
		{
			let c = body_buf_ptr.unwrap();
			match member_ptr
			{
				0 => record.id.push(c),
				1 => parent_id.push(c),
				2 => response_id.push(c),
				3 => record.nick_name.push(c),
				4 => record.mail.push(c),
				5 => record.mail_md5.push(c),
				6 => record.create_at.push(c),
				_ => record.comment.push(c)
			}
		}
		body_buf_ptr = body_buf_iter.next();
	}
	if !parent_id.is_empty(){
		record.parent_id = Some(parent_id);
	}
	if !response_id.is_empty(){
		record.response_id = Some(response_id);
	}
	println!(
		"HttpServer::ResponseSelector::RequestBodyTackle() => Done"
	);
	Some(record)
}

fn DatabaseQuery(mut stream: &mut TcpStream,has_query: bool,query: &str,request_content: &RequestContent)
{
	println!(
		"HttpServer::ResponseSelector::DatabaseQuery() => ..."
	);
	let database_table = DatabaseTable::new();

	if database_table.is_none()
	{
		println!(
			"HttpServer::ResponseSelector::DatabaseQuery() => \
			DatabaseTable::new() fail"
		);
		return;
	}

	let database_table = database_table.unwrap();
	if has_query
	{
		let file_type = "text/plain";
		let mut buf = String::new();

		match query.to_uppercase().as_str()
		{
			"COUNT" => {
				println!(
					"HttpServer::ResponseSelector::DatabaseQuery() => \
					query{{COUNT}}"
				);
				buf.push_str(&database_table.parent_record_count());
			},
			"PARENT" => {
				println!(
					"HttpServer::ResponseSelector::DatabaseQuery() => \
					query{{PARENT}}"
				);
				buf.push_str(&database_table.get_parent_record());
			},
			"REPLY" => {
				println!(
					"HttpServer::ResponseSelector::DatabaseQuery() => \
					query{{REPLY}}"
				);
				buf.push_str(&database_table.get_child_record());
			},
			"INSERT" => {
				println!(
					"HttpServer::ResponseSelector::DatabaseQuery() => \
					query{{INSERT}}"
				);
				if let Some(record) = 
				RequestBodyTackle(&request_content.req_body)
				{
					database_table.insert(&record);
					Response2xx::Response201(&mut stream);
				} else { Response5xx::Response500(&mut stream); }
			},
			_ => {
				println!(
					"HttpServer::ResponseSelector::DatabaseQuery() => \
					query{{UNKNOW}}"
				);
				Response4xx::Response400(&mut stream);
			}
		}
		Response2xx::Response200(
			&mut stream,
			&file_type,
			&buf.into_bytes()
		);
	}
	println!(
		"HttpServer::ResponseSelector::DatabaseQuery() => Done"
	);
}

fn FileQuery(mut stream: &mut TcpStream,path: &str)
{
	println!("HttpServer::ResponseSelector::FileQuery() => ...");
	if CONFIG.is_none()
	{
		println!(
			"HttpServer::ResponseSelector::FileQuery() => CONFIG read fail"
		);
		return;
	}

	let file = File::open(
		&UrlFormat(&format!("{}{}",&CONFIG.as_ref().unwrap().root_dir,path))
	);
	if !file.is_ok()
	{ 
		Response4xx::Response404(&mut stream);
		return; 
	}

	let mut file = file.unwrap();
	let mut file_type = String::new();
	let mut buf = Vec::new();
		
	match RequestType::GetRequestFileType(&path)
	{
		RequestFileType::HTML => file_type.push_str("text/html"),
		RequestFileType::CSS => file_type.push_str("text/css"),
		RequestFileType::JS => file_type.push_str("text/javascript"),
		RequestFileType::PNG => file_type.push_str("text/png"),
		RequestFileType::WASM => file_type.push_str("application/wasm"),
		_ => file_type.push_str("text/plain")
	}
	file.read_to_end(&mut buf).unwrap();

	Response2xx::Response200(&mut stream,&file_type,&buf);			
	println!("HttpServer::ResponseSelector::FileQuery() => Done");
}

pub fn ResponseSelector(mut stream: &mut TcpStream,request_content: &RequestContent)
{
	println!("HttpServer::ResponseSelector::ResponseSelector() => ...");
	let url_buf = &request_content.req_url;
	let mut path = String::new();
	let mut query = String::new();
	let mut switch_on = false;
		
	for c in url_buf.chars().rev()
	{
		if '?' == c 
		{ 
			switch_on = true; 
			continue;
		}
		if switch_on { path.insert(0,c); }
		else { query.insert(0,c); }
	}

	if false == switch_on{
		std::mem::swap(&mut path,&mut query);
	}	
	match RequestType::GetRequestPathType(&path)
	{
		RequestPathType::DATABASE => {
			DatabaseQuery(
				&mut stream,
				switch_on,	
				&query,
				&request_content
			);
		},
		RequestPathType::API => {
			Response2xx::Response201(&mut stream);
		},
		RequestPathType::FILE => {
			FileQuery(&mut stream,&path);
		},
		_ => Response4xx::Response400(&mut stream)
	}
	println!("HttpServer::ResponseSelector::ResponseSelector() => Done");
}
