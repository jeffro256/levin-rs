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
		
		let header_read_size = stream.as_ref().unwrap().read(&mut buf)?;
		println!("Read {}", header_read_size);
		
		//let payload_read_size = stream.as_ref().unwrap().read(&mut buf[header_read_size..])?;
		//println!("Read 2");
		//let total_read_size = header_read_size + payload_read_size;
		let total_read_size = header_read_size;

		if total_read_size != 0 {
			outf.write_all(&buf[..total_read_size])?;
			outf.write_all(&divider)?;
			outf.sync_all()?;

			println!("wrote 1");

			let section: serde_epee::Section = serde_epee::from_bytes(&mut &buf[33..])?;
			println!("{:?}", section);
		}
	}

	Ok(())
}

fn main() -> levin::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:38080")?;
	let mut buf_stream = BufWriter::new(stream);

	let bucket = levin::handshake::create_handshake_request_bucket(0, 0x32498724324242)?;
	bucket.send(&mut buf_stream)?;
	buf_stream.flush()?;

	println!("wrote output");

	/*let mut working_bucket_inf = File::open("listener_dump.dat")?;
	let mut working_bucket = [0u8; 275];
	working_bucket_inf.read_exact(&mut working_bucket)?;
	buf_stream.write_all(&working_bucket)?;
	buf_stream.flush()?;*/

	let mut stream = buf_stream.into_inner().unwrap();
	let mut got_handshake = false;
	while !got_handshake {
		let mut recv_header = levin::header::Header::from_reader(std::io::Read::by_ref(&mut stream))?;
		println!("got header: {:?}", recv_header);

		let section: serde_epee::Section = serde_epee::from_reader(std::io::Read::by_ref(&mut stream))?;
		println!("got payload {:?}", section);

		got_handshake = recv_header.command() == levin::constants::LEVIN_COMMAND_HANDSHAKE;
	}

	//let mut f = File::create("bucket_test.dat")?;
	//bucket.send(&mut f)?;

	//listen_and_dump()?;

	Ok(())
}
