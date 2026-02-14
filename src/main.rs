//! user imports
use std::{
    io::{prelude::*, BufReader}, // the std lib offers a module to listen to a TCP connection
    net::{TcpListener, TcpStream},
};

const LCL_ADDR: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(LCL_ADDR).unwrap(); // bind kind works like new

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //unwrap because we know that the connection will

        //be established for certain ( we cold also use ?)
        println!("Connection established"); //if i load the browser after start runing the program,
                                            //this will be printed
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
}
