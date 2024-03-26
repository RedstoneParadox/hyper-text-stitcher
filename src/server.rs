use std::net::TcpListener;

pub fn start_dev_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}