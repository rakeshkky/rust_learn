fn main() {
    let mut s1 = String::from("123");
    s1.push_str("456");
    print_length(&s1); // Ownership of s1 is borrowed by the print_length function through the reference.
    s1.push_str("789"); // Works now
    println!("Final string is {}", s1);
}

fn print_length(s: &String) -> () {
    println!("Size of string, {}, is {}", s, s.len());
}
