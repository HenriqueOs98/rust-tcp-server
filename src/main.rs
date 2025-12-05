use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


const IP: &str = "127.0.0.1";

const PORT: &str = "8181";

fn handle_client(mut stream: TcpStream) {
    use std::io::Read;

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer)
        .expect("Failed to read stream");

    println!("Received {} byes.", bytes_read);

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 17\r\n\r\nHello from Rust!\n";

    stream.write(response.as_bytes())
        .expect("Failed to write on stream");

    stream.flush()
        .expect("Failed to free write buffer");

    println!("Response sent and connection terminated.");

}


fn main() -> std::io::Result<()> {   

    println!("Server listening on {}:{}", IP, PORT);

    let listener = TcpListener::bind(&format!("{}:{}", IP, PORT))?;


    for stream in listener.incoming(){
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Error while accepting the connection {}", e),
        }
    }
    Ok(())
}


