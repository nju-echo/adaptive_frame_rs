use std::net::TcpStream;

pub trait TCP {
    fn get_socket(&self) -> &TcpStream;
    fn send(&mut self, str: &str) -> Result<(), std::io::Error>;
    fn recv(&mut self) -> Result<String, std::io::Error>;
    fn close(&mut self) -> Result<(), std::io::Error>;
    fn callback(&self);
    fn set_lock_flag(&self, lock_flag: bool);

    //useless for lock will be automatically released when out of scope
    //fn unlock(&self);
}
