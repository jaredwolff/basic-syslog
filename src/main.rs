use std::net::UdpSocket;
use std::str;

fn main() {
    let s = UdpSocket::bind("0.0.0.0:10514").unwrap();
    let mut buf = [0u8; 2048];
    loop {
        let (data_read, _) = s.recv_from(&mut buf).unwrap();
        let msg = str::from_utf8(&buf[0..data_read]).unwrap();

        print!("{}", msg);
    }
}
