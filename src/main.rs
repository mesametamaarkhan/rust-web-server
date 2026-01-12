use std::{
    fs, // standard library filesystem
    io::{BufReader, prelude::*}, // get traits and types that let us read and write to stream
    net::{TcpListener, TcpStream},
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

    // only need first line of request to validate the request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        // success message data 
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // write_all takes a &[u8] and sends the bytes directly down the connection
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        // success message data 
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // write_all takes a &[u8] and sends the bytes directly down the connection
        stream.write_all(response.as_bytes()).unwrap();
    }

    

    

}