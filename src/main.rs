extern crate rmp_serde;

fn main() {
    env_logger::init();
    let mode = std::env::args().nth(1).expect("You should specify '--client' or '--server' mode.");
    match mode.as_str() {
        "--client" => socket_check::do_the_job(socket_check::Mode::Client),
        "--server" => socket_check::do_the_job(socket_check::Mode::Server),
        _ => panic!("Wrong arguments"),
    }
}

mod socket_check {
    pub enum Mode {Client, Server}
    const IP_ADDR_AND_PORT: &str = "127.0.0.1:34254";

    pub fn do_the_job(mode: Mode) -> () {
        match mode {
            Mode::Client => client::do_the_clients_job(),
            Mode::Server => server::start_server(),
        }
    }

    mod data_structure {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct User {
            pub name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Message  {
            pub autor: User,
            pub data: String
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Topic {
            pub name: String,
            pub messages: Vec<Message>
        }
    }

    pub fn dev_topic() -> data_structure::Topic {
        use data_structure::*;
        let u1 = User{name: String::from("Pawel")};
        let u2 = User{name: String::from("Tomek")};
        let m1 = Message{autor: u1, data: String::from("Some example message")};
        let m2 = Message{autor: u2, data: String::from("Other example message")};
        let t1 = Topic{name: String::from("Example topic"), messages: vec![m1, m2]};
        t1
    }

    mod client {
        use std::io::Write;

        pub fn do_the_clients_job() -> () {
            let mut stream = std::net::TcpStream::connect(super::IP_ADDR_AND_PORT).unwrap();
        //    stream.write_all(b"some_bytes").unwrap();
            let example = super::dev_topic();
            let message_pack_buffer: Vec<u8> = rmp_serde::to_vec(&example).unwrap();
            let bytes_send = stream.write(&message_pack_buffer).unwrap();
            log::info!("{} was send to server", bytes_send);
        }
    
    }
    mod server {
        use super::data_structure::*;
        use std::io::Read;

        pub fn start_server() {
            log::info!("Server started");
            let listener = std::net::TcpListener::bind(super::IP_ADDR_AND_PORT).unwrap();
            for stream in listener.incoming() {
                let mut stream =  stream.unwrap();
                log::info!("stream {:?} was accepted", stream);
                let mut income_buffer: Vec<u8> = Vec::new();
                let bytes_read = stream.read_to_end(&mut income_buffer).unwrap(); // why read_to_end instead just read?
                log::info!("{} bytes was read", bytes_read);
                let message: Topic = rmp_serde::from_slice(&income_buffer).unwrap();
                log::info!("send message is: {:?}", message);
            }
            log::info!("Server stopped");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::socket_check::*;

    #[test]
    fn dummy_test(){
        println!("from dummy test");
        initialize_logger();
        start_server_thread();
        //std::thread::sleep(std::time::Duration::from_secs(3));
        //println!("should be in sleep for 3 secs");
        do_the_job(Mode::Client);
    }

    fn start_server_thread() {
        std::thread::spawn( || {
            log::debug!("The test is about to start server");
            do_the_job(Mode::Server);
        });
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    fn initialize_logger() {
        env_logger::init();
    }

}