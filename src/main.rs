use std::thread;

use std::vec::Vec;

use std::io::Read;
use std::io::Write;

use std::net::Shutdown;
use std::net::TcpStream;
use std::net::TcpListener;


mod dbcore;

fn client_recv(mut stream: TcpStream)
{
	println!("Got a new client");
	let mut buf = [0 as u8; 4096];
	let mut vecbuf: Vec<u8> = Vec::new();
	while match stream.read(&mut buf)
	{
		Ok(sz) =>
		{
			println!("Read a buffer of size {}", sz);
			for c in buf.iter()
			{
				vecbuf.push(*c);
			}

			if sz == 0
			{
				for c in vecbuf.iter()
				{
					if *c != 0
					{
						print!("{}", *c as char);
					}
				}
				return ();
			}
			true
		},

		Err(e) =>
		{
			println!("Unable to read: {}", e);
			stream.shutdown(Shutdown::Both).unwrap();
			false
		},
	} {}
}

fn main() 
{
	let listener = TcpListener::bind("0.0.0.0:8180").unwrap();
	let acceptor = listener.incoming();
	dbcore::initdb();
	println!("Starting server...");

	for incoming in acceptor
	{
		thread::spawn( || 
		{
			match incoming
			{
				Ok(incoming) =>
				{
					client_recv(incoming);
				}

				Err(e) =>
				{
					println!("Error encountered: {}", e);
				}
			}
		});
	}
}
