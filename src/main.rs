use std::net::{SocketAddr, UdpSocket};
use tinydns::{build_query, TYPE_A};

fn main() {
    let query = build_query("www.example.com", TYPE_A);
    let ip_address: SocketAddr = "8.8.8.8:53"
        .parse()
        .expect("Failed to parse the server address");
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.send_to(&query, ip_address).unwrap();

    let mut buf = [0; 1024];

    let result = socket.recv_from(&mut buf);

    match result {
        Ok((bytes_recv, src_addr)) => {
            println!("Recv bytes {}", bytes_recv);
            println!("Addr {}", src_addr);
            println!("Recv {:x?}", &buf[..bytes_recv]);
        }
        Err(e) => {
            eprintln!("Error in receiving response! {}", e);
        }
    }
}
