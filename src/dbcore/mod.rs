extern crate rusqlite;

use rusqlite::Connection;
use rusqlite::Result;
use rusqlite::NO_PARAMS;


use std::fs;



pub fn initdb()
{
	std::fs::create_dir_all(".projman/").unwrap();
	let conn = Connection::open(".projman/core.db").unwrap();
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_core (ID BIGINT PRIMARY KEY UNIQUE, Name TEXT NOT NULL UNIQUE)",
		NO_PARAMS
	).unwrap();
	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_issues (IssueID BIGINT PRIMARY KEY, IssueName TEXT NOT NULL, 
			IsOpen BOOLEAN, IssueFiled DATETIME NON NULL, IssueClosed DATETIME, IssuePriority SMALLINT)", 
		NO_PARAMS
	).unwrap();
	
	conn.execute(
		"CREATE TABLE IF NOT EXISTS projects_contents (ID BIGINT PRIMARY KEY UNIQUE, IssueID BIGINT UNIQUE, ProjectPriority SMALLINT)", 
			NO_PARAMS
	).unwrap();
}
