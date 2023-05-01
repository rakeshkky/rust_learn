fn main() {
    let len = {
        let s1 = String::from("123");
        // Below line transfers ownership of Heap from s1 to s2
        let mut s2 = s1; // s1 is invalidated here. So using of s1 after causes compile errors.
        s2.push_str("456");
        // s1.push_str("456"); causes compile error
        s2.len()
    };
    println!("{}", len);
}
