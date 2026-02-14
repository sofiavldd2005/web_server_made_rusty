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
    fs, //file system stuff
    io,
    io::{prelude::*, BufReader}, // the std lib offers a module to listen to a TCP connection
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server_made_rusty::{PoolCreationError, ThreadPool};

const LCL_ADDR: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(LCL_ADDR).unwrap(); // bind kind works like new
                                                         //
    let mut input = String::new(); // string to how user input
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read line");
    let size = input.trim().parse().expect("Please Type a Number!");

    let pool = match ThreadPool::new(size) {
        Ok(p) => {
            println!("Pool created with {size} threads.");
            p
        }
        Err(_) => {
            println!("Failed to create pool, defaulting to 1 thread.");
            ThreadPool::new(1).unwrap()
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); //wraps a reference to
                                              //the stream
                                              //lines colected into a vector
    let http_request = buf_reader
        .lines() //method that returns an iterator of type result<string, std::io:.Error>
        .next()
        .unwrap()
        .unwrap();

    let (status_line, path_filename) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/alo.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/alo.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    let contents = fs::read_to_string(path_filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
