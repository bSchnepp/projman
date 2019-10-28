use std::thread;

use std::vec::Vec;

use std::io::Read;
use std::io::Write;

use std::net::Shutdown;
use std::net::TcpStream;
use std::net::TcpListener;

fn client_recv(mut stream: TcpStream)
{
	println!("Got a new client");
	let mut buf = [0 as u8; 4096];
	while match stream.read(&mut buf)
	{
		Ok(sz) =>
		{
			println!("Read a buffer of size {}", sz);
			if sz == 0
			{
				for c in buf.iter()
				{
					if *c != 0
					{
						println!("{}", c);
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
	println!("Starting server...");

	for incoming in acceptor
	{
		thread::spawn( || 
		{
			match incoming
			{
				Ok(mut incoming) =>
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
