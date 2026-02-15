# web_server_made_rusty
A multi-threaded TCP/HTTP server built in Rust, following the implementation patterns from The Rust Programming Language (Chapter 20).

## Motivation
I am an Aerospace Engineer, and Embedded Software developer, who aims to pursue a masters in CS.
While I am comfortable with real-time constraints, memory-mapped I/O, and hardware protocols, I knew nothing about Computer networks.
In RED, IST's rocketry team, my focus is the Embedded Software running on our flight computer. The GroundStation, which hosts a web-server for a dashboard to monitor the mission is equally important. 
Becoming The Software Team leader, I found necessary to also learn more about Computer networks in order to have a better vision of how to implement the GroudStation backend.

This project serves as way to learn a little about computer networks, while also practising my rust Programming skills :).

## The concepts explored

1. **TCP/IP Stack**
  1. TCP Handshake: Implementing the logic that manages reliable, ordered data delivery.
  2. The 5-Tuple: Understanding how the OS identifies unique connections via Source/Dest IP, Ports, and Protocol.

2. **HTTP/1.1 Protocol**
  1. I Implemented a manual parser for HTTP requests. This taught me that web communication is essentially just a specific "packet format" (headers vs. body) delimited by `\r\n\r\n`.
  2. Header Precision: Learning the hard way that a single typo (like Content-lenght vs Content-Length) can break the entire "telemetry" stream to the browser.
  3. Status Codes: Mapping server states to standard HTTP responses (200 OK, 404 Not Found).

3. **Concurrent Architecture (Thread Pooling)**
  1. This server, uses a Thread Pool to handle simultaneous requests.
  2. The 
```Rust 
  Arc<Mutex<mpsc::Receiver>>
```  Combining Atomic Reference Counting and Mutual Exclusion to safely distribute jobs accros worker threads without data races. I found this to be particular different from C( the lang i usually use at RED), especially the fact that the Mutex is automaticaly "Unlocked", whereas in C, i have to manualy unlock the mutex.
  3. Using closures (`FnOnce`) and `Box` to pass tasks efficiently.

## The Project Structure (As of now ...)

1. `main.rs`: The entry point. Handles the TCP Listener and CLI input for pool sizing.

2. `lib.rs`: The "Engine." Contains the ThreadPool and Worker implementations.

3. html/: The payload directory containing the web assets.

## If you want to run this

### Prerequisites

  1. Rust (2021 onwards)
  2. Git 

### Instalation and Execution

1. **Clone the repo** : 
```Bash
git clone https://github.com/sofiavldd2005/web_server_made_rusty.git
cd web_server_made_rusty
```
2. **Run**:  `cargo run`

3. Configure the thread pool : give a number of threads

4. As of now, go to your browser and navigate to `http://127.0.0.1:7878`

This project is inspired by the final project in "The Rust Programming Language" book, customized to explore the intersection of aerospace logic and web networking.
More is comming to improve starting from this projet.
