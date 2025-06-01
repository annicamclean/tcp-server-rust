//importing necessary modules from Rust library
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
	//this is a buffer to get data from the client
	let mut buffer = [0; 1024];
	//this line read data from the stream and stores it in the buffer
	stream.read(&buffer).expect("Failed to read from client!");
	
	
	let request = String::from_utf8_lossy(&buffer[..]);
}

