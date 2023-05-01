// Data Types
struct User {
    name: String,
    email: String,
    age: i32,
}

fn main() {
    // constructing a User value
    let mut my_user = User {
        name: String::from("Rakesh"),
        email: String::from("random@random.com"),
        age: 27
    };
    my_user.name = "Rakesh Updated".to_string();
    println!("my_user name is {}, age is {} and email is {}", my_user.name, my_user.age, my_user.email);
}
