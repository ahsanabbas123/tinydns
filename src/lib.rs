use rand::Rng;
use std::vec::Vec;

pub const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let id_bytes = self.id.to_be_bytes();
        let flags_bytes = self.flags.to_be_bytes();
        let num_que_bytes = self.num_questions.to_be_bytes();
        let num_ans_bytes = self.num_answers.to_be_bytes();
        let num_auth_bytes = self.num_authorities.to_be_bytes();
        let num_additionals_bytes = self.num_additionals.to_be_bytes();

        let dns_header_bytes: Vec<u8> = [
            id_bytes,
            flags_bytes,
            num_que_bytes,
            num_ans_bytes,
            num_auth_bytes,
            num_additionals_bytes,
        ]
        .concat();

        return dns_header_bytes;
    }

    pub fn parse(buf: &[u8]) -> DNSHeader {
        return DNSHeader {
            id: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            flags: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            num_questions: u16::from_be_bytes(buf[4..6].try_into().unwrap()),
            num_answers: u16::from_be_bytes(buf[6..8].try_into().unwrap()),
            num_authorities: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
            num_additionals: u16::from_be_bytes(buf[10..12].try_into().unwrap()),
        };
    }
}

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub type_: u16,
    pub class_: u16,
}

impl DNSQuestion {
    pub fn to_bytes(&self) -> Vec<u8> {
        let name_bytes = self.name.clone().into_bytes();
        let type_bytes = self.type_.to_be_bytes().to_vec();
        let class_bytes = self.class_.to_be_bytes().to_vec();
        return [name_bytes, type_bytes, class_bytes].concat();
    }

    pub fn parse(buf: &[u8], mut index: usize) -> (usize, DNSQuestion) {
        let mut parts: Vec<String> = Vec::new();
        let mut l: usize = buf[index].into();
        while l != 0 {
            index += 1;
            let part: String = String::from_utf8((&buf[index..index + l]).to_vec()).unwrap();
            index += l;
            parts.push(part);
            l = buf[index].into();
        }
        let name = parts.join(".");
        index += 1;
        return (
            index + 4,
            DNSQuestion {
                name: name,
                type_: u16::from_be_bytes(buf[index..index + 2].try_into().unwrap()),
                class_: u16::from_be_bytes(buf[index + 2..index + 4].try_into().unwrap()),
            },
        );
    }
}

#[derive(Debug)]
pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSRecord>,
    pub authorities: Vec<DNSRecord>,
    pub additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    pub fn parse(buf: &[u8]) -> DNSPacket {
        let mut index: usize = 0;

        let header: DNSHeader = DNSHeader::parse(&buf);
        index += 12;

        let mut questions: Vec<DNSQuestion> = Vec::new();
        for _ in 0..header.num_questions {
            let question: DNSQuestion;
            (index, question) = DNSQuestion::parse(&buf, index);
            questions.push(question);
        }

        let mut answers: Vec<DNSRecord> = Vec::new();
        for _ in 0..header.num_answers {
            let ans: DNSRecord;
            (index, ans) = DNSRecord::parse(&buf, index);
            answers.push(ans);
        }

        let mut authorities: Vec<DNSRecord> = Vec::new();
        for _ in 0..header.num_authorities {
            let auth: DNSRecord;
            (index, auth) = DNSRecord::parse(&buf, index);
            authorities.push(auth);
        }

        let mut additionals: Vec<DNSRecord> = Vec::new();
        for _ in 0..header.num_additionals {
            let adds: DNSRecord;
            (index, adds) = DNSRecord::parse(&buf, index);
            additionals.push(adds);
        }

        return DNSPacket {
            header: header,
            questions: questions,
            answers: answers,
            authorities: authorities,
            additionals: additionals,
        };
    }
}

#[derive(Debug)]
pub struct DNSRecord {
    pub name: String,
    pub type_: u16,
    pub class_: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl DNSRecord {
    pub fn parse(buf: &[u8], mut index: usize) -> (usize, DNSRecord) {
        let name: String;
        (index, name) = decode_dns_name(buf, index);
        let type_: u16 = u16::from_be_bytes(buf[index..index + 2].try_into().unwrap());
        let class_: u16 = u16::from_be_bytes(buf[index + 2..index + 4].try_into().unwrap());
        let ttl: u32 = u32::from_be_bytes(buf[index + 4..index + 8].try_into().unwrap());

        let data_len: u16 = u16::from_be_bytes(buf[index + 8..index + 10].try_into().unwrap());
        index += 10;

        let l: usize = data_len.into();
        let data: Vec<u8> = buf[index..index + l].to_vec();
        index += l;

        return (
            index,
            DNSRecord {
                name: name,
                type_: type_,
                class_: class_,
                ttl: ttl,
                data: data,
            },
        );
    }
}

pub fn decode_dns_name(buf: &[u8], mut index: usize) -> (usize, String) {
    let mut parts: Vec<String> = Vec::new();
    let compression_bits: u8 = 0b1100_0000;
    let mut l: u8 = buf[index];
    index = index + 1;
    while l != 0 {
        if l & compression_bits != 0 {
            let res: String;
            (index, res) = decode_compressed_dns_name(buf, l, index);
            parts.push(res);
            break;
        } else {
            let len: usize = l.into();
            parts.push(String::from_utf8((&buf[index..index + len]).to_vec()).unwrap());
            index += len;
        }
        l = buf[index];
        index = index + 1;
    }
    return (index, parts.join("."));
}

pub fn decode_compressed_dns_name(buf: &[u8], l: u8, mut index: usize) -> (usize, String) {
    let mut top6bits: u8 = l & 0b0011_1111;
    top6bits += buf[index];
    index += 1;
    let pointer: usize = top6bits.into();
    let (_, res) = decode_dns_name(buf, pointer);
    return (index, res);
}

pub fn encode_dns_name(domain_name: &str) -> String {
    let mut encoded: String = String::from("");
    let delimiter = '.';
    for part in domain_name.split(delimiter) {
        let part_len = part.len() as u8;
        let encoded_part = String::from_utf8(part_len.to_be_bytes().to_vec()).unwrap() + part;
        encoded.push_str(&encoded_part);
    }
    encoded.push_str(&String::from_utf8((0 as u8).to_be_bytes().to_vec()).unwrap());
    return encoded;
}

pub fn build_query(domain_name: &str, record_type: u16) -> Vec<u8> {
    let name: String = encode_dns_name(domain_name);
    let id: u16 = rand::thread_rng().gen_range(1..65535);
    let recursion_desired: u16 = 1 << 8;
    let header = DNSHeader {
        id: id,
        num_questions: 1,
        flags: recursion_desired,
        num_answers: 0,
        num_additionals: 0,
        num_authorities: 0,
    };
    let question = DNSQuestion {
        name: name,
        type_: record_type,
        class_: CLASS_IN,
    };
    let mut query_bytes = header.to_bytes();
    let q_bytes = question.to_bytes();
    query_bytes.extend(q_bytes);

    return query_bytes;
}
