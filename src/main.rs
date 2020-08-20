

fn main() {
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
            //let mut stream = std::net::TcpStream::connect("127.0.0.1:12123").unwrap();
            //let some_tytes = b"huehe bytes";
            //stream.write(&[1]).unwrap()
            let mut stream = std::net::TcpStream::connect("127.0.0.1:34254").unwrap();

            stream.write(&[1]);
        }
    }
    mod server {
        pub fn start_server() {
            let listener = std::net::TcpListener::bind("127.0.0.1:12123").unwrap();
            for stream in listener.incoming() {
                println!("stream {:?} was accepted", stream.unwrap());
            }
        }
    }

}

#[allow(dead_code)]
mod first_experiments {
    use std::fmt;
    fn check_disscussion_thread_with_str_slice_with_lifetime() {
        let mut discussion = DiscussionThread{topic: "bania bania"};
        println!("doscussion: {:?}", discussion);
        discussion.topic = "szaszlyk";
        println!("doscussion: {:?}", discussion);
    
        let mut ss = "some string slice";
        println!("{}",ss);
        ss = "another string slice";
        println!("{}",ss);
    }
    
    fn check_how_implementation_of_display_works() -> () {
        let some_vector: Vec<String> = vec![String::from("qwe"), String::from("asd"), String::from("zxc")];
        println!("{:?}", some_vector);
        let first_user = User{name: String::from("Pawelek"),
                            surname: String::from("Ziesasecki"),
                            messages: vec![String::from("kurde"), String::from("sorry"), String::from("zapomnialem")]};
        println!("Let's try print all at once, Debug:\n{:?}", first_user);
        println!("Here is non 'Debug' message:\n{}", first_user);
    }
    
    #[derive(Debug)]
    struct DiscussionThread<'a> {
        topic: &'a str
    }
    
    #[derive(Debug)]
    struct User {
        name: String,
        surname: String,
        messages: Vec<String>
    }
    
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = fmt::format(format_args!("{} {}\nPowiedzial:\n", self.name, self.surname));
            for message in self.messages.iter() {
                s.push_str(message);
                s.push_str("\n");
            }
            write!(f, "{}\n", s)
        }
    }
}