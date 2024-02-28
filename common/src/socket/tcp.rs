use std::net::TcpStream;

pub trait TCP {
    fn get_socket(&self) -> &TcpStream;
    fn send(&self, str: &str) -> bool;
    fn recv(&self) -> Option<String>;
    fn close(&self);
    fn callback(&self);
    fn set_lock_flag(&self, lock_flag: bool);
    fn unlock(&self);
    //useless for lock will be automatically released when out of scope
    //fn unlock(&self);
}
