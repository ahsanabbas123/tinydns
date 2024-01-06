use rand::Rng;
use std::vec::Vec;

pub const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

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
}

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
