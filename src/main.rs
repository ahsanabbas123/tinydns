use std::net::{SocketAddr, UdpSocket};
use tinydns::{build_query, DNSHeader, DNSPacket, DNSQuestion, DNSRecord, TYPE_A};

fn main() {
    let query = build_query("www.example.com", TYPE_A);
    let ip_address: SocketAddr = "8.8.8.8:53"
        .parse()
        .expect("Failed to parse the server address");
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    println!("Sending query to {}", ip_address);
    socket.send_to(&query, ip_address).unwrap();

    let mut buf = [0; 1024];

    let result = socket.recv_from(&mut buf);

    match result {
        Ok((bytes_recv, src_addr)) => {
            println!("{} bytes received", bytes_recv);
            println!("Response Source Addr {}", src_addr);
            println!("Response Bytes {:x?}", &buf[..bytes_recv]);

            let mut index: usize = 0;
            println!("Parsed Header {:x?}", DNSHeader::parse(&buf));
            index += 12;
            let question: DNSQuestion;
            (index, question) = DNSQuestion::parse(&buf, index);
            println!("Parsed Question {:x?}", question);

            let record: DNSRecord;
            (_, record) = DNSRecord::parse(&buf, index);
            println!("Parsed record {:x?}", record);

            let packet: DNSPacket = DNSPacket::parse(&buf);
            println!("Parsed packet {:x?}", packet);
        }
        Err(e) => {
            eprintln!("Error in receiving response! {}", e);
        }
    }
}
