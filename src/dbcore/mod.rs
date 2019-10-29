extern crate crypto;
extern crate rusqlite;


use crypto::sha3::Sha3;
use crypto::digest::Digest;

use rusqlite::Connection;
use rusqlite::Result;
use rusqlite::NO_PARAMS;

struct User
{
	name: String,
	hash: String,	/* sha3 hash of password */
}

impl User
{
	pub fn check_hash(&mut self, given_pass: &str) -> bool
	{
		let mut hasher = Sha3::sha3_512();
		hasher.input(given_pass.as_bytes());
		/* TODO: Implement salting. */
		return hasher.result_str() == self.hash;
	}
}



pub fn initdb()
{
	/* If *any* of this fails, we should panic anyway. */
	std::fs::create_dir_all(".projman/").unwrap();
	/* 
		We'll use the amazing (input == 0) hash of great 1 bit
		strength until I decide between sha256, sha3, or something
		else.
	 */
	let conn = Connection::open(".projman/core.db").unwrap();	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_users 
			(UserID BIGINT PRIMARY KEY UNIQUE, 
			UserName TEXT NOT NULL UNIQUE, 
			JoinDate DATETIME NOT NULL,
			PassHash TEXT NOT NULL,
			Administrator BOOLEAN);",
		NO_PARAMS
	).unwrap();
	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_contents 
			(ID BIGINT PRIMARY KEY UNIQUE, 
			ProjectPriority SMALLINT);", 
			NO_PARAMS
	).unwrap();
	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_issues 
			(IssueID BIGINT PRIMARY KEY,
			ProjectID BIGINT,
			IssueOpener BIGINT, 
			IssueName TEXT NOT NULL, 
			IsOpen BOOLEAN, 
			IssueFiled DATETIME NOT NULL, 
			IssueClosed DATETIME, 
			IssuePriority SMALLINT,
			FOREIGN KEY(IssueOpener) 
				REFERENCES projects_users(UserID),
			FOREIGN KEY(ProjectID) 
				REFERENCES projects_core(ID));", 
		NO_PARAMS
	).unwrap();
	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_issue_thread 
			(ID BIGINT PRIMARY KEY UNIQUE, 
			Name TEXT NOT NULL UNIQUE);",
		NO_PARAMS
	).unwrap();
}




mod test
{
	use super::*;
	
	#[test]
	fn check_hash()
	{
		let mut pass = "password";
		let mut hasher = Sha3::sha3_512();
		hasher.input((&pass[..]).as_bytes());
		
		let mut user = User
		{
			name: String::from("hi"),
			hash: String::from(hasher.result_str()),
		};
		
		assert_eq!(user.check_hash(pass), true);
	}
	
	#[test]
	fn check_hash_neq()
	{
		let mut pass = "passw0rd?";
		let mut hasher = Sha3::sha3_512();
		hasher.input("password".as_bytes());
		
		let mut user = User
		{
			name: String::from("hi"),
			hash: String::from(hasher.result_str()),
		};
		
		assert_eq!(user.check_hash(pass), false);
	}
}
