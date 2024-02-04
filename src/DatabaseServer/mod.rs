use super::Config::CONFIG;
use serde::{ Deserialize,Serialize };

use mysql::{ 
	OptsBuilder,
	Pool,
	from_row,
	prelude::Queryable
};

#[derive(Debug,Deserialize,Serialize)]
pub struct DatabaseTable
{
	pub host: String,
	pub user: String,
	pub password: String,
	pub db_name: String,
	pub tb_name: String
} 

#[derive(Debug,Clone)]
pub struct Record
{
	pub id: String,
	pub parent_id: Option<String>,
	pub response_id: Option<String>,
	pub nick_name: String,
	pub mail: String,
	pub mail_md5: String,
	pub create_at: String,
	pub comment: String
}

impl DatabaseTable
{
	pub fn new() -> Option<Self>
	{
		println!(
			"DatabaseServer::DatabaseTable::new() => ..."
		);
		if CONFIG.is_none()
		{
			println!(
				"DatabaseServer::DatabaseTable::new() => CONFIG read fail"
			);
			return None;
		}

		let database_config = &(CONFIG.as_ref().unwrap().database_config);

		println!(
			"DatabaseServer::DatabaseTable::new() => Done"
		);
		Some(
			Self{
				host: database_config.host.clone(),
				user: database_config.user.clone(),
				password: database_config.password.clone(),
				db_name: database_config.db_name.clone(),
				tb_name: database_config.tb_name.clone()
			}
		)
	}

	fn is_null(&self) -> bool
	{
		println!(
			"DatabaseServer::DatabaseTable::is_null() => ..."
		);
		let opts = OptsBuilder::new()
			.ip_or_hostname(Some(&self.host))
			.user(Some(&self.user))
			.pass(Some(&self.password))
			.db_name(Some(&self.db_name))
		;

		if let Ok(pool) = Pool::new(opts)
		{
			if let Ok(mut conn) = pool.get_conn()
			{
				let query = &format!(
					"SELECT COUNT(*) FROM information_schema.tables \
					WHERE table_schema = '{}' \
					AND table_name = '{}';",
					&self.db_name,&self.tb_name
				);
				println!(
					"DatabaseServer::DatabaseTable::is_null() => \
					query => \n{}",&query
				);

				let query_result:Option<String> = conn.query_first(
					&query
				).unwrap();

				println!(
					"DatabaseServer::DatabaseTable::is_null() => {:?}",
					&query_result
				);
				if query_result.is_some()
				{
					if "0" != query_result.unwrap(){	
						return false;
					} 
				}
			}	
		}
		true
	}

	fn table_create(&self)
	{
		println!(
			"DatabaseServer::DatabaseTable::table_create() => ..."
		);
		let opts = OptsBuilder::new()
			.ip_or_hostname(Some(&self.host))
			.user(Some(&self.user))
			.pass(Some(&self.password))
			.db_name(Some(&self.db_name))
		;
		if let Ok(pool) = Pool::new(opts)
		{
			if let Ok(mut conn) = pool.get_conn()
			{
				let query = format!(
					"CREATE TABLE {}(\
					id CHAR(8) PRIMARY KEY,\
					parent_id CHAR(8),\
					response_id CHAR(8),\
					nick_name CHAR(64),\
					mail CHAR(64),\
					mail_md5 CHAR(65),\
					create_at CHAR(16),\
					comment TEXT\
					);",&self.tb_name
				);
					
				println!(
					"DatabaseServer::DatabaseTable::table_create() => \
					query => \n{}",&query
				);
				conn.query_drop(&query).unwrap();
			}
		}
	}

	fn record_count(&self) -> String 
	{
		println!(
			"DatabaseServer::DatabaseTable::record_count() => ..."
		);
		let opts = OptsBuilder::new()
			.ip_or_hostname(Some(&self.host))
			.user(Some(&self.user))
			.pass(Some(&self.password))
			.db_name(Some(&self.db_name))
		;
		if let Ok(pool) = Pool::new(opts)
		{
			if let Ok(mut conn) = pool.get_conn()
			{
				let query = format!(
					"SELECT COUNT(*) FROM {};",&self.tb_name
				);
				println!(
					"DatabaseServer::DatabaseTable::record_count() => \
					query => \n{}",&query
				);

				let query_result:Option<String> = conn.query_first(
					&query
				).unwrap();

				if query_result.is_some(){ 
					return query_result.unwrap(); 
				}
			}
		}
		"0".to_string()
	}

	pub fn parent_record_count(&self) -> String 
	{
		println!(
			"DatabaseServer::DatabaseTable::parent_record_count() => ..."
		);
		if false == self.is_null()
		{
			let opts = OptsBuilder::new()
				.ip_or_hostname(Some(&self.host))
				.user(Some(&self.user))
				.pass(Some(&self.password))
				.db_name(Some(&self.db_name))
			;
			if let Ok(pool) = Pool::new(opts)
			{
				if let Ok(mut conn) = pool.get_conn()
				{
					let query = &format!(
						"SELECT COUNT(*) FROM {} \
						WHERE parent_id IS NULL \
						AND response_id IS NULL;",
						&self.tb_name
					);
					println!(
						"DatabaseServer::DatabaseTable::\
						parent_record_count() => query => \n{}",&query
					);

					let query_result:Option<String> = conn.query_first(
						&query
					).unwrap();

					if query_result.is_some(){ 
						return query_result.unwrap(); 
					}
				}
			}
		}
		"0".to_string()
	}

	pub fn insert(&self,record: &Record)
	{
		println!(
			"DatabaseServer::DatabaseTable::insert() => ..."
		);
		let mut count_id = String::from("0");
		if true == self.is_null(){
			self.table_create();
		} else { count_id = self.record_count(); }

		let opts = OptsBuilder::new()
			.ip_or_hostname(Some(&self.host))
			.user(Some(&self.user))
			.pass(Some(&self.password))
			.db_name(Some(&self.db_name))
		;
		if let Ok(pool) = Pool::new(opts)
		{
			if let Ok(mut conn) = pool.get_conn()
			{
				let mut query = String::new();

				query.push_str(
					&format!("INSERT INTO {}",&self.tb_name)
				);
				query.push_str(
					"(id,parent_id,response_id,nick_name,mail,mail_md5,\
					create_at,comment) "
				);
				query.push_str("VALUES(");
				query.push_str(
					&format!("'{}',",
						(count_id.parse::<i32>().unwrap() + 1).to_string()
					)
				);
				if record.parent_id.is_none() && record.response_id.is_none(){
					query.push_str("NULL,NULL,");
				} 
				else 
				{
					query.push_str(
						&format!("'{}','{}',",
							record.parent_id.clone().unwrap(),
							record.response_id.clone().unwrap()
						)
					);
				}
				query.push_str(
					&format!("'{}','{}','{}','{}','{}');",
						&record.nick_name,&record.mail,&record.mail_md5,
						&record.create_at,&record.comment
					)
				);

				println!(
					"DatabaseServer::DatabaseTable::insert() => \
					query => \n{}",&query
				);
				conn.query_drop(&query).unwrap();
			}
		}
	}

	pub fn get_parent_record(&self) -> String
	{
		println!(
			"DatabaseServer::DatabaseTable::get_parent_record() => ..."
		);

		let mut response_body = String::new();
		if false == self.is_null()
		{
			let opts = OptsBuilder::new()
				.ip_or_hostname(Some(&self.host))
				.user(Some(&self.user))
				.pass(Some(&self.password))
				.db_name(Some(&self.db_name))
			;

			if let Ok(pool) = Pool::new(opts)
			{
				if let Ok(mut conn) = pool.get_conn()
				{
					let query = &format!(
						"SELECT * FROM {} \
						WHERE parent_id IS NULL \
						AND response_id IS NULL \
						ORDER BY create_at;",
						&self.tb_name
					);

					println!(
						"DatabaseServer::DatabaseTable::\
						get_parent_record() => query => \n{}",&query
					);
					conn.query_iter(&query)
						.unwrap()
						.for_each(|row|{
							let r: 
								(String,Option<String>,Option<String>,String,
								String,String,String,String)
							= from_row(row.unwrap());

							response_body.push_str(&r.0);
							if r.1.is_some() && r.2.is_some()
							{
								response_body.push_str(
									&format!(":{}:{}",
										r.1.unwrap(),r.2.unwrap()
									)
								);
							} else{ response_body.push_str("::"); }
									
							response_body.push_str(
								&format!(
									":{}:{}:{}:{}:{},",
									r.3,r.4,r.5,r.6,r.7
								)
							);	
						});
				}
			}
			println!(
				"DatabaseServer::DatabaseTable::get_parent_record() => \n{}",
				&response_body
			);
		}
		response_body
	}

	pub fn get_child_record(&self) -> String
	{
		println!(
			"DatabaseServer::DatabaseTable::get_child_record() => ..."
		);

		let mut response_body = String::new();
		if false == self.is_null()
		{
			let opts = OptsBuilder::new()
				.ip_or_hostname(Some(&self.host))
				.user(Some(&self.user))
				.pass(Some(&self.password))
				.db_name(Some(&self.db_name))
			;

			if let Ok(pool) = Pool::new(opts)
			{
				if let Ok(mut conn) = pool.get_conn()
				{
					let query = &format!(
						"SELECT * FROM {} \
						WHERE parent_id IS NOT NULL \
						AND response_id IS NOT NULL \
						ORDER BY create_at",
						&self.tb_name
					);

					println!(
						"DatabaseServer::DatabaseTable::\
						get_child_record() => query => \n{}",&query
					);
					conn.query_iter(&query)
					.unwrap()
					.for_each(|row|{
						let r: 
							(String,Option<String>,Option<String>,String,
							String,String,String,String)
						= from_row(row.unwrap());

						response_body.push_str(&r.0);
						if r.1.is_some() && r.2.is_some()
						{
						response_body.push_str(
							&format!(":{}:{}",
								r.1.unwrap(),r.2.unwrap()
							)
						);
						} else{ response_body.push_str("::"); }
							
						response_body.push_str(
							&format!(
								":{}:{}:{}:{}:{},",
								r.3,r.4,r.5,r.6,r.7
							)
						);	
					});
				}
			}
			println!(
				"DatabaseServer::DatabaseTable::get_parent_record() => \n{}",
				&response_body
			);
		}
		response_body
	}
}
