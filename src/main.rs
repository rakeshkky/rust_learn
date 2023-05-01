// Data Types

// A tuple struct
struct User(String, String, i32);

fn main() {
    // constructing a User value
    let my_user = User(String::from("Rakesh"), String::from("random@random.com"), 27);
    println!("my_user name is {}, age is {} and email is {}", my_user.0, my_user.2, my_user.1);
}
