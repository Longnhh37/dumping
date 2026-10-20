use std::net::UdpSocket;

use sockets::prompt_address;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for a server binding.
    let (host, port) = prompt_address("Enter a server address (e.g. 0.0.0.0):")?;

    // Open a socket for incoming messages.
    let server_socket = UdpSocket::bind((host.as_str(), port))?;
    println!("Waiting for messages...");

    // Read messages that appear in the socket.
    let mut buf = [0u8; 1024];
    loop {
        let (n, client_address) = server_socket.recv_from(&mut buf)?;
        if n == 0 {
            break;
        }
        let message = String::from_utf8_lossy(&buf[..n]).replace('\n', "");
        println!("[{client_address}]: [{message}]");

        // If a message is "QUIT", terminate the server.
        if message == "QUIT" {
            break;
        }
    }
    Ok(())
}
