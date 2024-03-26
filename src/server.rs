use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use httparse::Request;
use crate::build_site;

pub fn start_dev_server() {
    build_site();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = Request::new(&mut headers);
        let mut buf = vec![];

        stream.read(&mut buf).expect("Unable to read from TCP stream.");
        request.parse(&buf).expect("Unable to parse HTTP request");

        println!("{:?}", request.path.unwrap());


        println!("Connection established!");
    }
}