use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

use common::socket::abstract_tcp::AbstractTCP;
use common::socket::tcp::TCP;

fn start_server(addr: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    for stream in listener.incoming().take(1) {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(), std::io::Error> {
    let mut tcp = AbstractTCP::build(stream, true)?;
    // Handle communication with the client using `tcp`
    // For example, read a message and echo it back
    let mut buf = [0; 1024];
    let bytes_read = tcp.get_socket().read(&mut buf)?;
    println!(
        "Server Received message: {}",
        String::from_utf8_lossy(&buf[..bytes_read])
    );

    //echo it back
    tcp.send("Hello, client!")?;
    Ok(())
}

fn start_client(addr: &str) -> Result<(), std::io::Error> {
    let stream = TcpStream::connect(addr)?;
    let mut tcp = AbstractTCP::build(stream, true)?;

    //thread spawn

    // Send a message to the server
    tcp.send("Hello, server!")?;

    // Receive a response from the server
    let response = tcp.recv()?;
    println!("Received from server: {}", response);

    Ok(())
}

#[test]
fn test_tcp_communication() {
    let server_addr = "127.0.0.1:7878";
    println!("Starting server on {}", server_addr);

    // Start the server in a new thread
    let server_thread = thread::spawn(move || {
        start_server(server_addr).expect("Server failed");
    });

    // Give the server a moment to start up
    thread::sleep(std::time::Duration::from_millis(100));

    // Start the client and connect to the server
    let client_thread = thread::spawn(move || {
        start_client(server_addr).expect("Client failed");
    });

    // Wait for both threads to complete
    server_thread.join().expect("Server thread panicked");
    client_thread.join().expect("Client thread panicked");
}
