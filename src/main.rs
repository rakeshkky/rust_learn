fn main() {
    let mut s1 = String::from("123");
    s1.push_str("456");
    print_length(s1); // Ownership of s1 is transfered to the print_length function. Heap memory is freed.
    // Using s1 again here causes compile error
    // s1.push_str("456"); error
}

fn print_length(s: String) -> () {
    let mut s_mut = s;
    s_mut.push_str("789");
    println!("Size of string, {}, is {}", s_mut, s_mut.len());
}
