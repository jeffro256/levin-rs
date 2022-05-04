use std::io::prelude::*;
use std::io::{Write, BufWriter};
use std::fs::File;
use std::net::{TcpListener, TcpStream};

extern crate levin;

fn listen_and_dump() -> levin::Result<()> {
	let listener = TcpListener::bind("localhost:48080")?; 

	let divider = "################".as_bytes();

	let mut outf = File::create("listener_dump.dat")?;

	for stream in listener.incoming() {
		let mut buf = [0u8; 4096];
		let read_size = stream.as_ref().unwrap().read(&mut buf)?;

		if read_size != 0 {
			outf.write_all(&buf[..read_size])?;
			outf.write_all(&divider)?;
			outf.sync_all()?;

			let section: serde_epee::Section = serde_epee::from_bytes(&mut &buf[33..])?;
			println!("{:?}", section);
		}
	}

	Ok(())
}

fn main() -> levin::Result<()> {
	let bucket = levin::handshake::create_handshake_request_bucket(0, String::new(), 0x32498724324242)?;

	//let mut stream = TcpStream::connect("127.0.0.1:38080")?;
	//let mut buf_stream = BufWriter::new(stream);
	//bucket.send(&mut buf_stream)?;
	//let section = serde_epee::deserialize_from(&mut buf_stream.into_inner().unwrap())?;
	//println!("{:?}", section);


	//let mut f = File::create("bucket_test.dat")?;
	//bucket.send(&mut f)?;

	listen_and_dump()?;

	Ok(())
}
