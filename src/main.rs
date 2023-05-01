fn main() {
    let k = 1;
    let j = {
        // This k is different from outside k since the scopes are different
        let k = 2;
        println!("K is {} inside scope", k);
        k +2
    };
    println!("J is {}", j);
    println!("K is {} outside scope", k);
}
