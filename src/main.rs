use std::fmt;

fn main() {

    let some_vector: Vec<String> = vec![String::from("qwe"), String::from("asd"), String::from("zxc")];
    println!("{:?}", some_vector);
    let first_user = User{name: String::from("Pawelek"),
                          surname: String::from("Ziesasecki"),
                          messages: vec![String::from("kurde"), String::from("sorry"), String::from("zapomnialem")]};
    println!("Let's try print all at once, Debug:\n{:?}", first_user);
    println!("Here is non 'Debug' message:\n{}", first_user);
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