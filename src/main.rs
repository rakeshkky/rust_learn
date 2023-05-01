// Data Types
enum Contact {
    Email(String),
    PhoneNumber(String),
}

impl Contact {
    fn to_string(&self) -> String {
        contact_to_string(self) // Or we can use match
    }
}

struct User {
    name: String,
    age: Option<i32>,
    contact: Contact,
}

fn main() {
    // constructing a User value
    let mut my_user = User {
        name: String::from("Rakesh"),
        age: Option::None,
        contact: Contact::Email(String::from("random@random.com")),
    };
    // Using implementation to_string() on Contact
    let contact_string = my_user.contact.to_string();
    let age_string = age_to_string(&my_user.age);
    println!("my_user name is {}, age is {} and a way of contact is {}", my_user.name, age_string, contact_string);
    my_user.contact = Contact::PhoneNumber(String::from("9632587410"));
    // Using function contact_to_string();
    println!("Contact is {}", contact_to_string(&my_user.contact));
    my_user.age = Option::Some(27);
    println!("Age is {}", age_to_string(&my_user.age));
}

fn contact_to_string(c: &Contact) -> String {
    match c {
        Contact::Email(email) => {
            format!("email, {}", email)
        },
        Contact::PhoneNumber(phone_number) => {
            format!("phone number, {}", phone_number)
        }
    }
}

fn age_to_string(age: &Option<i32>) -> String {
    match age {
        Option::Some(age_num) => {
            format!("{}", age_num)
        },
        Option::None => {
            "None".to_string()
        }
    }
}
