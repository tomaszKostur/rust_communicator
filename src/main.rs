extern crate rmp_serde;

fn main() {
    println!("heavy refactor, use tests instead");
    //env_logger::init();
    //let mode = std::env::args().nth(1).expect("You should specify '--client' or '--server' mode.");
    //match mode.as_str() {
    //    "--client" => communicator::do_the_job(communicator::Mode::Client),
    //    "--server" => communicator::do_the_job(communicator::Mode::Server),
    //    _ => panic!("Wrong arguments"),
    //}
}

mod communicator {
    //pub enum Mode {Client, Server}
    //const IP_ADDR_AND_PORT: &str = "127.0.0.1:34254";

    //pub fn do_the_job(mode: Mode) -> () {
    //    match mode {
    //        Mode::Client => client::do_the_clients_job(),
    //        Mode::Server => server::start_server(),
    //    }
    //}

    //mod data_structure {
    //    use serde::{Serialize, Deserialize};
//
    //    #[derive(Debug, Serialize, Deserialize)]
    //    pub struct User {
    //        pub name: String,
    //    }
//
    //    #[derive(Debug, Serialize, Deserialize)]
    //    pub struct Message  {
    //        pub autor: User,
    //        pub data: String
    //    }
//
    //    #[derive(Debug, Serialize, Deserialize)]
    //    pub struct Topic {
    //        pub name: String,
    //        pub messages: Vec<Message>
    //    }
    //}

    //pub fn dev_topic() -> data_structure::Topic {
    //    use data_structure::*;
    //    let u1 = User{name: String::from("Pawel")};
    //    let u2 = User{name: String::from("Tomek")};
    //    let m1 = Message{autor: u1, data: String::from("Some example message")};
    //    let m2 = Message{autor: u2, data: String::from("Other example message")};
    //    let t1 = Topic{name: String::from("Example topic"), messages: vec![m1, m2]};
    //    t1
    //}
//
    //pub mod client {
    //    use std::io::Write;
    //    use std::net::TcpStream;
//
    //    pub struct Client {
    //        //ip_addr_and_port_: String,
    //        stream_to_server_: TcpStream,
    //    }
//
    //    pub fn create(ip_and_port: String) -> Client {
    //        let  stream = TcpStream::connect(ip_and_port).expect("Couldnt connect to message_server");
    //        Client{/*ip_addr_and_port_: ip_and_port, */stream_to_server_: stream}
    //    }
//
    //    impl Client {
    //        pub fn dummy_method(&self) {
    //            log::debug!("from Client's dummy method, my ip and port is");
    //        }
    //    }
//
    //    pub fn do_the_clients_job() -> () {
    //        let mut stream = std::net::TcpStream::connect(super::IP_ADDR_AND_PORT).unwrap();
    //    //    stream.write_all(b"some_bytes").unwrap();
    //        let example = super::dev_topic();
    //        let message_pack_buffer: Vec<u8> = rmp_serde::to_vec(&example).unwrap();
    //        let bytes_send = stream.write(&message_pack_buffer).unwrap();
    //        log::info!("{} was send to server", bytes_send);
    //    }
    //
    //}

    pub mod server {
        //use super::data_structure::*;
        use std::io::Read;
        use std::net::{TcpListener, TcpStream};
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        use std::io::ErrorKind;

        pub struct Interface {
            topic_thread: std::thread::JoinHandle<()>,
            command_pipe: mpsc::Sender<InterfaceCommands>,
        }

        enum InterfaceCommands {
            Start,
            Stop,
        }

        impl Interface {
            pub fn start(&self) {
                self.command_pipe.send(InterfaceCommands::Start);
            }
            pub fn stop(self) {
                self.command_pipe.send(InterfaceCommands::Stop);
                self.topic_thread.join();
            }
            pub fn new(ip_and_port: &str) -> Interface {
                let (tx, rx) =  mpsc::channel::<InterfaceCommands>();
                let listener = std::net::TcpListener::bind(ip_and_port).expect("Cannot create TcpListener");
                listener.set_nonblocking(true).expect("Cannot set nonblocking listener");
                let topic_thread = TopicWorker{command_receiver: rx, message_listener: listener};
                let worker_loop = move || {topic_thread.topic_loop()};
                let topic_thread = thread::spawn(worker_loop);
                Interface{topic_thread: topic_thread, command_pipe: tx}
            }
        }

        struct TopicWorker {
            command_receiver: mpsc::Receiver<InterfaceCommands>,
            message_listener: TcpListener,
        }
        impl TopicWorker {
            fn topic_loop(&self) {
                loop {
                    println!("loop iter");
                    let command = self.command_receiver.try_recv();
                    match command {
                        Ok(command) => {
                            match command {
                                InterfaceCommands::Stop => break,
                                _ => self.command_handle(command),
                            }
                        },
                        Err(_) => self.regular_msg_receieve(),
                    }
                    thread::sleep(Duration::from_millis(100)); // performance save
                }
            }
            fn regular_msg_receieve(&self)  {
                println!("from regular_msg_receieve");
                //let (mut stream, _addr) = self.message_listener.accept().expect("Couldn't accept incoming connection");
                let stream  = self.message_listener.accept();
                match stream {
                    Ok(s) => {self.handle_incoming_message(s.0)},
                    Err(e) if e.kind()== ErrorKind::WouldBlock => { println!("would block") },
                    Err(e) => panic!("encountered IO error: {}", e)
                    }
                //log::info!("stream {:?} was accepted", stream);
                //let mut income_buffer: Vec<u8> = Vec::new();
                //let bytes_read = stream.read(&mut income_buffer).unwrap(); // why read_to_end instead just read?
                //println!("{} bytes was read", bytes_read);
                //let message: Topic = rmp_serde::from_slice(&income_buffer).unwrap();
                //log::info!("send message is: {:?}", message);
            }
            fn command_handle(&self, command: InterfaceCommands) {
                match command {
                    InterfaceCommands::Start => self.handle_start_command(),
                    InterfaceCommands::Stop => panic!("Unexpected match to command handler"),
                }
            }
            fn handle_start_command(&self) {
                println!("start command handler");
            }
            fn handle_incoming_message(&self, mut stream: TcpStream) {
                println!("handle_incoming_message");
                let mut buf: Vec<u8> = Vec::new();
                stream.read(&mut buf);
                println!("after read");
            }

        }
        ///////////////////////

        //pub struct Server {
        //    server_listener_: TcpListener,
        //}

        //pub fn create(ip_and_port: String) -> Server {
        //    let listener = std::net::TcpListener::bind(ip_and_port).expect("Cannot create Server");
        //    Server{server_listener_: listener, listening_: false}
        //}

        //impl Server {
        //    pub fn listen_forever(self) {
        //        let listening_closure = move || {
        //            while self.listening_{
        //                let (mut stream, _addr) = self.server_listener_.accept().expect("Couldn't accept incoming connection");
        //                //let mut stream =  stream.unwrap();
        //                log::info!("stream {:?} was accepted", stream);
        //                let mut income_buffer: Vec<u8> = Vec::new();
        //                let bytes_read = stream.read_to_end(&mut income_buffer).unwrap(); // why read_to_end instead just read?
        //                log::info!("{} bytes was read", bytes_read);
        //                let message: Topic = rmp_serde::from_slice(&income_buffer).unwrap();
        //                log::info!("send message is: {:?}", message);
        //            }
        //        };
        //        std::thread::spawn(listening_closure);
        //    }
        //    pub fn stop_listening(&mut self) {
        //        self.listening_ = false;
        //    }
        //}
//
        //pub fn start_server() {
        //    log::info!("Server started");
        //    let listener = std::net::TcpListener::bind(super::IP_ADDR_AND_PORT).unwrap();
        //    for stream in listener.incoming() {
        //        let mut stream =  stream.unwrap();
        //        log::info!("stream {:?} was accepted", stream);
        //        let mut income_buffer: Vec<u8> = Vec::new();
        //        let bytes_read = stream.read_to_end(&mut income_buffer).unwrap(); // why read_to_end instead just read?
        //        log::info!("{} bytes was read", bytes_read);
        //        let message: Topic = rmp_serde::from_slice(&income_buffer).unwrap();
        //        log::info!("send message is: {:?}", message);
        //    }
        //    log::info!("Server stopped");
        //}
    }
}

#[cfg(test)]
mod tests {
    use super::communicator::*;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    const IP_ADDR_AND_PORT: &str = "127.0.0.1:34254";

    #[test]
    fn test_servers_interface() {
        let topic = server::Interface::new(IP_ADDR_AND_PORT);
        topic.start();

        let mut stream = std::net::TcpStream::connect(IP_ADDR_AND_PORT).unwrap();
        stream.write_all(b"some_bytes").unwrap();
        thread::sleep(Duration::from_secs(1));
        //let example = super::dev_topic();
        //let message_pack_buffer: Vec<u8> = rmp_serde::to_vec(&example).unwrap();
        //let bytes_send = stream.write(&message_pack_buffer).unwrap();
        //log::info!("{} was send to server", bytes_send);

        topic.stop();
    }

    //#[test]
    //fn test_basic_communication(){
    //    initialize_logger();
    //    start_server_thread();
    //    std::thread::sleep(std::time::Duration::from_millis(20)); // Sleep to be sure that server is allready up
    //    do_the_job(Mode::Client);
    //}

    //#[test]
    //fn test_client_impl(){
    //    initialize_logger();
    //    start_server_thread();
    //    let client_instance = client::create(String::from(IP_ADDR_AND_PORT));
    //    client_instance.dummy_method();
    //}

    //#[test]
    //fn test_start_server() {
    //    let server_instance = server::create(String::from(IP_ADDR_AND_PORT));
    //    let instance_ref = &server_instance;
    //    std::thread::spawn(|| {se.listen_forever()});
    //    std::thread::sleep(std::time::Duration::from_millis(200));
    //    instance_ref.stop_listening();
    //}
//
    //// test utils
    //fn start_server_thread() {
    //    std::thread::spawn( || {
    //        do_the_job(Mode::Server);
    //    });
    //    std::thread::sleep(std::time::Duration::from_millis(200));
    //}
//
    //fn initialize_logger() {
    //    let _= env_logger::try_init();
    //}

    //struct Test_Environment{
    //    logger_initialized: bool
    //}
    //impl Test_Environment {
    //    pub fn initialize_logger(&mut self) {
    //        if !self.logger_initialized {
    //            env_logger::init();
    //            self.logger_initialized = true;
    //        }
    //    }
    //}

}