use std::io;
use std::net::TcpStream;

use log::error;

use crate::socket::abstract_tcp::AbstractTCP;

pub trait TCP {
    fn super_reference(&self) -> &AbstractTCP;
    fn send_result(&self, str: &str) -> Result<(), io::Error> {
        self.super_reference().send_result(str)
    }
    fn recv_result(&self) -> Result<String, io::Error> {
        self.super_reference().recv_result()
    }

    fn send_err_handle(&self) {
        self.callback();
        self.unlock();
        match self.get_socket().shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                error!("shutdown tcp error while send err handling : {}", e);
            }
        }
    }

    fn recv_err_handle(&self) {
        self.callback();
        match self.get_socket().shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(e) => {
                error!("shutdown tcp error while recv err handling : {}", e);
            }
        }
    }

    fn get_socket(&self) -> &TcpStream {
        self.super_reference().get_socket()
    }
    fn send(&self, str: &str) -> bool {
        match self.send_result(str) {
            Ok(_) => true,
            Err(e) => {
                error!("send error: {}", e);
                self.send_err_handle();
                false
            }
        }
    }
    fn recv(&self) -> Option<String> {
        match self.recv_result() {
            Ok(s) => {
                self.unlock();
                if s.is_empty() {
                    //todo: 特判不够robust，如果真的是空字符串，那么就会出现问题
                    error!("recv error: recv empty string, remote close connection.");
                    return None;
                }
                Some(s)
            }
            Err(e) => {
                error!("recv error: {}", e);
                self.recv_err_handle();
                self.unlock();
                None
            }
        }
    }
    fn close(&self);
    fn callback(&self) {
        self.super_reference().callback();
    }
    fn set_lock_flag(&self, lock_flag: bool) {
        self.super_reference().set_lock_flag(lock_flag);
    }
    fn unlock(&self) {
        self.super_reference().unlock();
    }
    //useless for lock will be automatically released when out of scope
    //fn unlock(&self);
}

/* todo: modify above to below structure
struct Base {
    …fields…
}

struct Derived {
    base: Base,
    other_field: u32,
}

trait Superable { // that's a superbly bad name
    fn super(&self) -> &Base;
    fn super_mut(&mut self) -> &mut Base;

    fn some_method(&self) -> u32 {
        // now you can use methods of Base from here
        self.super().other_method() + 1
    }
}

impl Superable for Derived {
    fn super(&self) -> &Base {
        &self.base
    }

    fn super_mut(&mut self) -> &mut Base {
        &mut self.base
    }
}
 */
