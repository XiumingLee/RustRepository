use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running tcp server on port 3000!");

    // 获取每一个连接流
    for stream in listener.incoming() {
        //let _stream = stream.unwrap();
        let mut stream = stream.unwrap();
        println!("Connection established!");

        // 读取客户端的发来的信息，并原样返回。
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        println!("The info is: {:?}",str::from_utf8(&buffer).unwrap());

        stream.write(&mut buffer).unwrap();
    }
}
