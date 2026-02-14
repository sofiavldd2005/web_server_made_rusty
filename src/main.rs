//! The HTTP Request
//! Format : Method Request-URI HTTP-Version CRLF
//! headers CRLF
//! message-body
//! The GET request : the client is asking for info
//!
//! Now onto the Writting a response
//! HTTP-Version Status-Code Reason-Phrase CRLF
//! headers CRLF
//! message-body
//! HTTP/1.1 200 OK\r\n\r\n -> Example of a response an "Ok" type one
//! user imports
use std::{
    fs,                          //file system stuff
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
    let buf_reader = BufReader::new(&stream); //wraps a reference to
                                              //the stream
    let http_request: Vec<_> = buf_reader // lines colected in a
        // vector
        .lines() //method that returns an iterator of type
        //result<string, std::io:.Error>
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");

    let status_line = "HTTP/1.1 200 OK"; //to say its "ok"

    let contents = fs::read_to_string("html/alo.html").unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/html\r\n\
    \r\n\
    {}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
