

struct UdpServer {

}

struct UdpListener {

}

use std::{net::{Ipv4Addr, UdpSocket}, str::from_utf8};

use log::{info, warn};

pub fn debug_udp_client() {
    info!("listen udp ip: 127.0.0.1 port: 5967");
    
    std::thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:5967").unwrap();
        
        match socket.send_to(b"Ping", "127.0.0.1:8080") {
            Ok(_) => info!("Сообщение отправилось"),
            Err(_) => warn!("Сообщение не отправилось"),
        }

        loop {
            let mut buf = [0u8; 1024];
            //socket.recv(&mut buf).unwrap();
            
            if buf != [0u8; 1024] {
                let len = from_utf8(&buf).unwrap().len();
                info!("Получил сообщение длинной: {:?}", len);
            }
            
        }
    });


}