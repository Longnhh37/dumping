use std::net::UdpSocket;

use tcp_udp::{prompt, prompt_address};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for a server address.
    let (host, port) = prompt_address("Enter a server address (e.g. 127.0.0.1):")?;

    // Open a socket to write messages (bind to an ephemeral local port).
    let client_socket = UdpSocket::bind("0.0.0.0:0")?;
    loop {
        // Capture a message.
        let message = prompt("Enter a message:")?;

        // If the message is "quit", terminate the client.
        if message == "quit" {
            break;
        }

        // Write the message through the socket.
        client_socket.send_to(message.as_bytes(), (host.as_str(), port))?;
    }
    Ok(())
}
