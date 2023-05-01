// Data Types
enum Contact {
    Email(String),
    PhoneNumber(String),
}

struct User {
    name: String,
    age: i32,
    contact: Contact,
}

fn main() {
    // constructing a User value
    let mut my_user = User {
        name: String::from("Rakesh"),
        age: 27,
        contact: Contact::Email(String::from("random@random.com")),
    };
    let contact_string = contact_to_string(&my_user.contact);
    println!("my_user name is {}, age is {} and a way of contact is {}", my_user.name, my_user.age, contact_string);
    my_user.contact = Contact::PhoneNumber(String::from("9632587410"));
    println!("Contact is {}", contact_to_string(&my_user.contact));
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
