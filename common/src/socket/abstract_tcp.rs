use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::socket::tcp::TCP;

//TODO: should lock AbstractTCP
//Why lock inside

///AbstractTCP is a wrapper of TcpStream
/// it's an abstract class
/// should not be used directly
/// used its child instead
#[derive(Debug)]
pub struct AbstractTCP {
    socket: TcpStream,
    lock: Arc<Mutex<()>>,
    lock_flag: AtomicBool,
    buf_out: BufWriter<TcpStream>,
    buf_in: BufReader<TcpStream>,
}

impl TCP for AbstractTCP {
    fn set_lock_flag(&self, flag: bool) {
        self.lock_flag.store(flag, Ordering::SeqCst);
    }

    fn send(&mut self, str: &str) -> Result<(), std::io::Error> {
        let _guard = if self.lock_flag.load(Ordering::SeqCst) {
            Some(self.lock.lock().unwrap())
        } else {
            None
        };
        let str = str.replace("\n", "//huanhang");
        self.buf_out.write_all((str + "\n").as_bytes())?;
        self.buf_out.flush()?;
        Ok(())
    }

    fn recv(&mut self) -> Result<String, std::io::Error> {
        let mut ret = String::new();
        self.buf_in.read_line(&mut ret)?;
        Ok(ret.replace("//huanhang", "\n"))
    }

    fn get_socket(&self) -> &TcpStream {
        &self.socket
    }

    fn close(&mut self) -> Result<(), std::io::Error> {
        self.socket.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }

    fn callback(&self) {
        //do nothing
    }
}

impl AbstractTCP {
    pub fn build(socket: TcpStream, lock_flag: bool) -> Result<Self, std::io::Error> {
        socket.set_nodelay(true)?;
        let lock = if lock_flag {
            Arc::new(Mutex::new(()))
        } else {
            Arc::new(Mutex::new(())) // You might want to handle this differently in Rust.
        };
        let out = BufWriter::new(socket.try_clone()?);
        let input = BufReader::new(socket.try_clone()?);

        Ok(AbstractTCP {
            socket,
            lock,
            lock_flag: AtomicBool::new(lock_flag),
            buf_out: out,
            buf_in: input,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    use super::*;

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
}
