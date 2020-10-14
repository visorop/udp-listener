mod user_params;

use crate::user_params::UserParams;
use std::env;
use std::net::UdpSocket;

const PORT: u32 = 9191;
const BUFFER_SIZE: u32 = 65_515;

fn main() {
    let args: Vec<String> = env::args().collect();

    let user_params: UserParams = UserParams::get_user_params(args);

    println!("Started with {}", user_params);

    let socket_name = format!("127.0.0.1:{}", user_params.port);
    println!("Binding to socket: {}", socket_name);

    let socket = UdpSocket::bind(socket_name).expect("Couldn't bind to socket.");

    println!("Bounded to socket successfully.");

    //let mut buf = [0; 65_515];
    let buffer_size = user_params.buffer_size as usize;
    let mut buf = Vec::with_capacity(buffer_size);
    buf.resize(buffer_size, 0);

    let mut count: u32 = 0;

    loop {
        let size = socket.recv(&mut buf).unwrap();
        let message = String::from_utf8_lossy(&buf[..size]);
        println!(
            "=== {} === Message with size {} bytes and content:\n{}\n===",
            count, size, message
        );
        count += 1;
    }
}
