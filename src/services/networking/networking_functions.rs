use std::{
    io::{prelude::*, BufReader, Error},
    net::{TcpListener, TcpStream},
    str::from_utf8
};
use crate::services::io_functions::output;

pub fn listen(callback: fn(String), address: &str) {
    let listener: TcpListener = TcpListener::bind(address).expect("TcpListener Binding");

    for stream in listener.incoming() {

        let stream: TcpStream = stream.expect("Initializing TcpStream");
        let request: String = handle_tcp_stream(stream).expect("Reading Stream Data");

        callback(request);

    }

}

pub fn send(message: &str, address: &str) {
    match TcpStream::connect(format!("{}:7878", address)) {
        Ok(mut stream) => {

            stream.write(format!("{}\n\n", message).as_bytes()).expect("Sending Data");

            let mut response: Vec<u8> = Vec::new();
            match stream.read_to_end(&mut response) {
                Ok(_) => {
                    let response_string: &str = from_utf8(&response).expect("String Conversion Error").trim_end();
                    if response_string != message.trim_end() {
                        output("Message confirmation failure, message may have been corrupted.");
                    };
                },
                Err(e) => {
                    output(format!("No response: {}", e));
                }
            }
        },
        Err(e) => {
            output(format!("Failed to connect: {}", e));
        }
    }
}

fn handle_tcp_stream(mut stream: TcpStream) -> Result<String, Error> {

    let mut reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let mut request: String = String::new();
    reader.read_line(&mut request).expect("Reading Line");

    stream.write_all(format!("{}\n\n", &request).as_bytes())?;

    return Ok(request);
}

