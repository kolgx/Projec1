pub mod tcp_communication_demo_model {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::thread;
    use std::time::Duration;

    const TCP_SERVER_ADDRESS: &str = "127.0.0.1";
    const TCP_SERVER_PORT: &str = "7878";

    pub fn tcp_server_start() {
        let listener:TcpListener = match TcpListener::bind(format!("{}:{}", TCP_SERVER_ADDRESS, TCP_SERVER_PORT)) {
            Ok(listener) => {
                println!("TCP server started, listening on port {}", TCP_SERVER_PORT);
                listener
            }
            Err(_) => {
                println!("Error occurred when binding TCP server!");
                return;
            }
        };
        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => {
                    println!("Error occurred when accepting connection!");
                    continue;
                }
            };
            match handle_connection(stream) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error occurred when handling connection! {}", e);
                    continue;
                }
            }
        }

    }

    fn handle_connection(mut stream: TcpStream) -> Result<(), &'static str> {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer){
            Ok(_) => {}
            Err(_) => {
                println!("Error occurred when reading from stream!");
                return Err("Error occurred when reading from stream!");
            }
        }
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "src/model/tcp_communication_demo.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "src/model/tcp_communication_demo.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/model/404.html")
        };
        let response = format!("{}{}", status_line, filename);
        match stream.write(response.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                return Err("Error occurred when writing to stream!");
            }
        }
        match stream.flush() {
            Ok(_) => {}
            Err(_) => {
                return Err("Error occurred when flushing stream!");
            }
        }
        return Ok(());
    }

    fn tcp_client_start() {
        let mut stream = match TcpStream::connect(format!("{}:{}", TCP_SERVER_ADDRESS, TCP_SERVER_PORT)) {
            Ok(stream) => {
                println!(format!("Connected to server {}:{}!", TCP_SERVER_ADDRESS, TCP_SERVER_PORT));
                stream
            },
            Err(_) => {
                println!("Error occurred when connecting to server!");
                return;
            }
        };
        match stream.write(b"GET / HTTP/1.1\r\n\r\n") {
            Ok(_) => {}
            Err(_) => {
                println!("Error occurred when writing to stream!");
                return;
            }
        }
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                println!("Error occurred when reading from stream!");
                return;
            }
        }
        println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
    }
}

