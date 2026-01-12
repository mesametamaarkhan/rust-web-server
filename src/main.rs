use std::net::TcpListener;

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
    }
}