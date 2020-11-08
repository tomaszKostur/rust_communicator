extern crate rmp_serde;

fn main() {
    println!("heavy refactor, use tests instead");
}

mod communicator {

    pub mod server {
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
                    //println!("loop iter");
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
                //println!("from regular_msg_receieve");
                let stream  = self.message_listener.accept();
                match stream {
                    Ok(s) => {self.handle_incoming_message(s.0)},
                    Err(e) if e.kind()== ErrorKind::WouldBlock => { (/* just do nothing and wait for another loop*/ ) },
                    Err(e) => panic!("encountered IO error: {}", e)
                    }
            }
            fn command_handle(&self, command: InterfaceCommands) {
                match command {
                    InterfaceCommands::Start => self.handle_start_command(),
                    InterfaceCommands::Stop => panic!("Unexpected match to command handler"), // the "Stop" command is allready handled in topic loop
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
        topic.stop();
    }
}