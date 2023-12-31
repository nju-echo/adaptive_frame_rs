use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref SOCKETS: Mutex<HashMap<u16, UdpSocket>> = {
        let mut map = HashMap::new();
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        map.insert(0, socket);
        Mutex::new(map)
    };
}

pub fn close(port: u16) {
    let mut sockets = SOCKETS.lock().unwrap();
    if let Some(socket) = sockets.remove(&port) {
        drop(socket); // Explicitly drop the socket
    }
}

pub fn send(client_address: &str, client_port: u16, msg: &str) {
    let sockets = SOCKETS.lock().unwrap();
    if let Some(socket) = sockets.get(&0) {
        let addr = format!("{}:{}", client_address, client_port);
        let _ = socket.send_to(msg.as_bytes(), &addr);
    }
}

pub fn send_with_ip(ip_addr: IpAddr, client_port: u16, msg: &str) {
    println!("aa");
    let sockets = SOCKETS.lock().unwrap();
    println!("ss");
    if let Some(socket) = sockets.get(&0) {
        let addr = SocketAddr::new(ip_addr, client_port);
        let _ = socket.send_to(msg.as_bytes(), &addr);
    }
}

///timeout is None when set timeout infinite
/// first recv will create a socket
/// #Warning:
/// should not recv the same udp socket in different threads.
/// its behavior is undefined.
pub fn recv(port: u16, timeout: Option<u64>) -> Option<String> {
    // 首先获取或创建套接字
    let socket = {
        let mut sockets = SOCKETS.lock().unwrap();
        let socket = sockets.entry(port).or_insert_with(|| {
            UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind to address")
        });
        //socket.set_nonblocking(false);
        if let Some(t) = timeout {
            let _ = socket.set_read_timeout(Some(std::time::Duration::from_millis(t)));
        } else {
            let _ = socket.set_read_timeout(None);
        }

        //question: clone is time consuming whether need to clone?
        //需求上recv是很久才会调用一次，所以不会有性能问题
        socket.try_clone().expect("Failed to clone socket")
    };

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => Some(String::from_utf8_lossy(&buf[..size]).to_string()),
        Err(err) => {
            println!("Failed to receive UDP message: {}", err);
            None
        }
    }
}

//question: 先send再recv，recv收不到? answer: 要在recv创建udp socket，否则会收不到
//question： RWLock为了避免写锁饥饿，会导致按顺序调用写锁和读锁？ answer: 使用clone，其实不会有同时的send和recv

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_udp_send_and_recv() {
        let port = 34254;
        let port1 = 12345;
        let msg_to_send = "Hello, UDP!";

        thread::sleep(Duration::from_millis(100));
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        send_with_ip(addr, port, msg_to_send);

        // 在新线程上启动接收器
        let receiver_thread = thread::spawn(move || {
            // 设置超时以避免测试无限期地等待
            let received_msg = recv(port, Some(1000)).expect("Failed to receive");
            assert_eq!(received_msg, msg_to_send);
        });

        let receiver_thread1 = thread::spawn(move || {
            // 设置超时以避免测试无限期地等待
            let received_msg = recv(port1, Some(1000)).expect("Failed to receive");
            assert_eq!(received_msg, msg_to_send);
        });
        // 给接收器一点时间来启动
        thread::sleep(Duration::from_millis(100));

        thread::sleep(Duration::from_millis(100));
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        send_with_ip(addr, port, msg_to_send);
        send_with_ip(addr, port, msg_to_send);
        // 准备地址并发送消息

        send_with_ip(addr, port1, msg_to_send);

        // 等待接收器线程完成
        receiver_thread.join().expect("Receiver thread panicked");
        receiver_thread1.join().expect("Receiver thread panicked");
    }
}
