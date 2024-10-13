use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::Rng;

fn handle_client(mut stream: TcpStream) {
    // Clone the stream for reading
    let read_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream for reading: {}", e);
            return;
        }
    };
    
    let mut reader = BufReader::new(read_stream);
    let mut buffer = String::new();

    // Wait for HELLO greeting
    if let Ok(_) = reader.read_line(&mut buffer) {
        buffer = buffer.trim().to_uppercase();
        if buffer != "HELLO" {
            if let Err(e) = writeln!(stream, "ERROR") {
                eprintln!("Failed to send ERROR: {}", e);
            }
            return;
        }

        // Generate a random number
        let random_number: u32 = rand::thread_rng().gen_range(1..=100);
        if let Err(e) = writeln!(stream, "NUM:{}", random_number) {
            eprintln!("Failed to send NUM: {}: {}", random_number, e);
            return;
        }

        buffer.clear();
        // Wait for the response
        if let Ok(_) = reader.read_line(&mut buffer) {
            buffer = buffer.trim().to_string();
            match buffer.parse::<u32>() {
                Ok(number) => {
                    if number == random_number * 2 {
                        if let Err(e) = writeln!(stream, "OK") {
                            eprintln!("Failed to send OK: {}", e);
                        }
                    } else {
                        if let Err(e) = writeln!(stream, "WRONG") {
                            eprintln!("Failed to send WRONG: {}", e);
                        }
                    }
                }
                Err(_) => {
                    if let Err(e) = writeln!(stream, "ERROR") {
                        eprintln!("Failed to send ERROR: {}", e);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read from stream.");
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Cannot start server.");
    println!("Server running on 0.0.0.0:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
