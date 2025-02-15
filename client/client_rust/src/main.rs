/*

    CLIENT

*/

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn read_messages(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    loop {

        match stream.read(&mut buffer) {
            
            // If the server sends a message
            Ok(bytes_read) if bytes_read > 0 => {
                
                let message = String::from_utf8_lossy(&buffer[..bytes_read]).trim_end_matches('\n').to_string();
                println!("{}", message);
            }

            // If the server closes the connection
            Ok(_) => {

                // Connection closed by the server
                println!("Server closed the connection.");
                // Close the entire program
                std::process::exit(0);
            }

            // If there is an error
            Err(_) => {
                // Handle errors
                println!("Error reading from server.");
                std::process::exit(0);
            }
        }
    }
}

fn main() -> io::Result<()> {

    let ip = "127.0.0.1";

    let porta = "8080";
    
    let mut stream = TcpStream::connect(ip.to_owned() + ":" + porta)?;
    println!("Connected to server at {}", ip);
    
    // Clone the stream for using in the read_messages function, because rust can only have one owner (function) of a resource
    let reader_stream = stream.try_clone()?;
    
    // Spawn a thread to read messages from the server
    thread::spawn( move || {

        print!("User: ");
        io::stdout().flush().unwrap(); // Flush stdout to make sure the print! macro is executed before reading input
    
        read_messages(reader_stream);
    });

    // Read user input and send it to the server
    let mut input= String::new();
    
    loop {

        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
        input.clear();
    }
}

