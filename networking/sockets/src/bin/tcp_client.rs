use std::{io::Write, net::TcpStream};

use sockets::{prompt, prompt_address};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for a server address.
    let (host, port) = prompt_address("Enter a server address (e.g. 127.0.0.1):")?;

    // Open a socket to write messages (closed automatically on drop).
    let mut client_socket = TcpStream::connect((host.as_str(), port))?;
    loop {
        // Capture a message.
        let message = prompt("Enter a message:")?;

        // If the message is "quit", terminate the client.
        if message == "quit" {
            break;
        }

        // Write the message through the socket.
        client_socket.write_all(message.as_bytes())?;

        // If the message is "QUIT", the server has been terminated.
        if message == "QUIT" {
            break;
        }
    }
    Ok(())
}
