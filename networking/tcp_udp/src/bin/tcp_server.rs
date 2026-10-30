use std::{io::Read, net::TcpListener};

use tcp_udp::prompt_address;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for a server binding.
    let (host, port) = prompt_address("Enter a server address (e.g. 0.0.0.0):")?;

    // Open a listener socket for incoming connections.
    let listener = TcpListener::bind((host.as_str(), port))?;
    loop {
        println!("Waiting for connections...");

        // Open a client socket for messages from this client.
        let (mut client_socket, client_address) = listener.accept()?;
        println!("Waiting for messages...");

        // Read messages that appear in the socket.
        let mut buf = [0u8; 1024];
        loop {
            let n = client_socket.read(&mut buf)?;
            if n == 0 {
                break;
            }
            let message = String::from_utf8_lossy(&buf[..n]).replace('\n', "");
            println!("[{client_address}]: [{message}]");

            // If a message is "QUIT", terminate the server.
            if message == "QUIT" {
                return Ok(());
            }
        }
    }
}
