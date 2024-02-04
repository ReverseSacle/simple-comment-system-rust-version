pub enum RequestMethodType
{
	GET,
	POST,
	PUT,
	DELETE,
	HEAD,
	OPTIONS,
	PATCH,
	CONNECT,
	TRACE,
	UNKNOW
}

pub enum RequestFileType
{
	HTML,
	CSS,
	JS,
	PNG,
	WASM,
	UNKNOW
}

pub enum RequestPathType
{
	FILE,
	DATABASE,
	API,
	UNKNOW
}

pub fn GetRequestMethodType(method_buf: &str)
-> RequestMethodType
{
	println!("HttpServer::RequestType::GetRequestMethodType() => ...");
	match method_buf.to_uppercase().as_str()
	{
		"GET" => {
			println!(
				"HttpServer::RequestType::GetRequestMethodType() => GET"
			);
			RequestMethodType::GET
		},
		"PUT" => {
			println!(
				"HttpServer::RequestType::GetRequestMethodType() => PUT"
			);
			RequestMethodType::PUT
		},
		"POST" => {
			println!(
				"HttpServer::RequestType::GetRequestMethodType() => POST"
			);
			RequestMethodType::POST
		},
		_ => { 
			println!(
				"HttpServer::RequestType::GetRequestMethodType() => UNKNOW"
			);
			RequestMethodType::UNKNOW
		}
	}		
}

pub fn GetRequestPathType(path_buf: &str)
-> RequestPathType
{
	println!("HttpServer::RequestType::GetRequestPathType() => ...");
	let mut buf = String::new();
			
	for c in path_buf.chars().rev()
	{
		if '/' == c{ break; }
		if '.' == c 
		{ 
			println!(
				"HttpServer::RequestType::GetRequestPathType() => File"
			);
			return RequestPathType::FILE; 
		}
		buf.insert(0,c);
	}
	match buf.to_uppercase().as_str()
	{
		"DATABASE" => {
			println!(
				"HttpServer::RequestType::GetRequestPathType() => DATABASE"
			);
			RequestPathType::DATABASE
		},
		"API" => {
			println!(
				"HttpServer::RequestType::GetRequestPathType() => API"
			);
			RequestPathType::API
		},
		_ => { 
			println!(
				"HttpServer::RequestType::GetRequestPathType() => UNKNOW"
			);
			RequestPathType::UNKNOW
		}
	}
}

pub fn GetRequestFileType(filename_buf: &str)
-> RequestFileType
{
	println!("HttpServer::RequestType::GetRequestFileType() => ...");
	let mut buf = String::new();

	for c in filename_buf.chars().rev()
	{
		if '.' == c { break; }
		buf.insert(0,c);
	}
	match buf.to_uppercase().as_str()
	{
		"HTML" => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => HTML"
			);
			RequestFileType::HTML
		}
		"CSS" => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => CSS"
			);
			RequestFileType::CSS
		}
		"JS" => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => JS"
			);
			RequestFileType::JS
		}
		"PNG" => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => PNG"
			);
			RequestFileType::PNG
		}
		"WASM" => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => WASM"
			);
			RequestFileType::WASM
		}
		_ => {
			println!(
				"HttpServer::RequestType::GetRequestFileType() => UNKNOW"
			);
			RequestFileType::UNKNOW
		}
	}
}
