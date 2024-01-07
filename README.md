## tinydns

A basic DNS resolver written in Rust, based on [dns-in-a-weekend](https://implement-dns.wizardzines.com/). 

#### Progress
```
➜  tinydns git:(main) ✗ cargo fmt && cargo run
   
   Compiling tinydns v0.1.0 (/Users/ahsan/code/tinydns)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/tinydns`

Sending query to 8.8.8.8:53
49 bytes received
Response Source Addr 8.8.8.8:53
Response Bytes [f6, 40, 81, 80, 0, 1, 0, 1, 0, 0, 0, 0, 3, 77, 77, 77, 7, 65, 78, 61, 6d, 70, 6c, 65, 3, 63, 6f, 6d, 0, 0, 1, 0, 1, c0, c, 0, 1, 0, 1, 0, 0, 27, 90, 0, 4, 5d, b8, d8, 22]
Parsed Header DNSHeader { id: f640, flags: 8180, num_questions: 1, num_answers: 1, num_authorities: 0, num_additionals: 0 }
Parsed Question DNSQuestion { name: "www.example.com", type_: 1, class_: 1 }
```