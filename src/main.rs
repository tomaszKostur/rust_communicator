

fn main() {
    let mode = std::env::args().nth(1).unwrap();
    match mode.as_str() {
        "--client" => socket_check::do_the_job(socket_check::Mode::Client),
        "--server" => socket_check::do_the_job(socket_check::Mode::Server),
        _ => panic!("Wrong arguments"),
    }
    socket_check::do_the_job(socket_check::Mode::Client);
}

mod socket_check {
    pub enum Mode {Client, Server}
    //const str IP_ADDR_AND_PORT = "127.0.0.1:12123"

    pub fn do_the_job(mode: Mode) -> () {
        match mode {
            Mode::Client => client::do_the_clients_job(),
            Mode::Server => server::start_server(),
        }
    }
    mod client {
        use std::io::prelude::*;
        pub fn do_the_clients_job() -> () {
            let mut stream = std::net::TcpStream::connect("127.0.0.1:34254").unwrap();
            stream.write_all(b"some_bytes").unwrap();
        }
    }
    mod server {
        pub fn start_server() {
            let listener = std::net::TcpListener::bind("127.0.0.1:34254").unwrap();
            for stream in listener.incoming() {
                use std::io::Read;
                let mut stream =  stream.unwrap();
                println!("stream {:?} was accepted", stream);
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer).unwrap();
                println!("send message is: {}", buffer);
            }
        }
    }

}