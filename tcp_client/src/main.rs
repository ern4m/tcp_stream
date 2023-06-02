use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Connect to the server
    let stream = TcpStream::connect("localhost:3333").expect("Failed to connect to server");
    let stream = Arc::new(Mutex::new(stream));

    // Clone the stream for use in the receive thread
    let receive_stream = Arc::clone(&stream);

    // Create a separate thread to receive server responses
    let receive_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let mut stream = receive_stream.lock().expect("Failed to acquire lock on stream");
            let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");
            if bytes_read == 0 {
                // Connection closed by the server
                break;
            }
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Server response: {}", response);
        }
    });

    // Main thread to send user input to the server
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let mut stream = stream.lock().expect("Failed to acquire lock on stream");
        stream
            .write(input.trim().as_bytes())
            .expect("Failed to write to server");
    }

    receive_thread.join().expect("Receive thread panicked");
}