use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};

use log::{error, log, trace};
use socket2::Socket;

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
    lock: (Mutex<bool>, Condvar),
    lock_flag: AtomicBool,
    buf_out: Option<RwLock<BufWriter<TcpStream>>>,
    buf_in: Option<RwLock<BufReader<TcpStream>>>,
}

impl TCP for AbstractTCP {
    fn get_socket(&self) -> &TcpStream {
        &self.socket
    }

    fn send(&self, str: &str) -> bool {
        if self.lock_flag.load(Ordering::SeqCst) {
            let (lock, cvar) = &self.lock;
            let mut guard = lock.lock().unwrap();
            while *guard {
                guard = cvar.wait(guard).unwrap();
            }
            *guard = true;
        }

        let str = str.replace("\n", "//huanhang");
        match self.buf_out {
            Some(ref buf_out) => {
                let mut buf_out = buf_out.write().expect("buf_out write lock failed");
                match buf_out.write_all((str + "\n").as_bytes()) {
                    Ok(_) => match buf_out.flush() {
                        Ok(_) => true,
                        Err(_) => {
                            self.send_err_handle();
                            false
                        }
                    },
                    Err(_) => {
                        self.send_err_handle();
                        false
                    }
                }
            }

            None => {
                panic!("buf_out is none");
            }
        }
    }

    fn recv(&self) -> Option<String> {
        let mut ret = None;
        let mut buf = String::new();
        match self.buf_in {
            Some(ref buf_in) => {
                match buf_in
                    .write()
                    .expect("buf_in write lock failed")
                    .read_line(&mut buf)
                {
                    Ok(_) => {
                        ret = Some(buf.replace("//huanhang", "\n"));
                    }
                    Err(_) => {
                        self.recv_err_handle();
                    }
                }
            }
            None => {
                panic!("buf_in is none");
            }
        }

        self.unlock();
        ret
    }

    fn close(&self) {
        match self.socket.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                log::error!("shutdown tcp error : {}", e);
            }
        }
    }

    fn callback(&self) {
        //do nothing
    }

    fn set_lock_flag(&self, flag: bool) {
        self.lock_flag.store(flag, Ordering::SeqCst);
    }

    fn unlock(&self) {
        if self.lock_flag.load(Ordering::SeqCst) {
            let (lock, cvar) = &self.lock;
            let mut guard = lock.lock().unwrap();
            *guard = false;
            cvar.notify_one();
        }
    }
}

const KEEPALIVE_TIME: u64 = (2 * 60 * 60);

impl AbstractTCP {
    pub fn new(socket: TcpStream, lock_flag: bool) -> Self {
        let socket2: Socket = socket.into();
        socket2
            .set_keepalive(Some(std::time::Duration::from_secs(KEEPALIVE_TIME)))
            .expect("set keepalive time failed");
        let socket = socket2.into_tcp_stream();

        //socket.set_nodelay(true);
        let lock = if lock_flag {
            (Mutex::new((false)), Condvar::new())
        } else {
            (Mutex::new((false)), Condvar::new()) // You might want to handle this differently in Rust.
        };

        let mut out = None;
        let mut input = None;
        match socket.try_clone() {
            Ok(socket_clone) => {
                out = Some(RwLock::new(BufWriter::new(socket_clone)));
            }
            Err(e) => {
                log::error!("clone tcp error : {}", e);
                match socket.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("shutdown tcp error : {}", e);
                    }
                }
            }
        }
        match socket.try_clone() {
            Ok(socket_clone) => {
                input = Some(RwLock::new(BufReader::new(socket_clone)));
            }
            Err(e) => {
                log::error!("clone tcp error : {}", e);
                match socket.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("shutdown tcp error : {}", e);
                    }
                }
            }
        }

        //let out = BufWriter::new(socket.try_clone()?);
        //let input = BufReader::new(socket.try_clone()?);

        AbstractTCP {
            socket,
            lock,
            lock_flag: AtomicBool::new(lock_flag),
            buf_out: out,
            buf_in: input,
        }
    }

    fn send_err_handle(&self) {
        self.callback();
        self.unlock();
        match self.socket.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                log::error!("shutdown tcp error while send err handling : {}", e);
            }
        }
    }

    fn recv_err_handle(&self) {
        self.callback();
        match self.socket.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                log::error!("shutdown tcp error while recv err handling : {}", e);
            }
        }
    }
}
