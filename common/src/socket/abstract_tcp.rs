use std::io::{BufRead, BufReader, BufWriter, Write};
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
    fn get_socket(&self) -> &TcpStream {
        &self.socket
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

    fn close(&mut self) -> Result<(), std::io::Error> {
        self.socket.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }

    fn callback(&self) {
        //do nothing
    }

    fn set_lock_flag(&self, flag: bool) {
        self.lock_flag.store(flag, Ordering::SeqCst);
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
