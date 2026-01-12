use std::{
    io::{BufReader, prelude::*}, // get traits and types that let us read and write to stream
    net::{TcpListener, TcpStream}, // 
};

fn main() {
    // bind returns new tcp listener instance (binds to a port)
    // returns Result<T,E> which means binding can fail sometimes
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  

    // incoming function returns iterator that gives a sequence of streams
    // a single stream represents open connection b/w client and server
    // read req from tcpstream and then write response to it
    // here we process each connection and produce series of streams to handle
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
        handle_connection(stream);
    }
}

// this function reads data from tcp stream 
// to see what data is being sent by the browser
fn handle_connection(mut stream: TcpStream) {
    // wraps a reference to the stream
    // this adds buffering by managing calls to the std::io:Read trait methods
    let buf_reader = BufReader::new(&stream);

    // collecting lines of request in a vector
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // success message data 
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // write_all takes a &[u8] and sends the bytes directly down the connection
    stream.write_all(response.as_bytes()).unwrap();

}