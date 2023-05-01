fn main() {
    let x = 31;
    println!("X is {}", x);
    let result = branch(x);
    println!("Result is {}", result);
}

fn branch(x : i32) -> i32 {
    if x > 40 {
        println!("X is greater than 40, mulitplying with 2");
        x * 2
    }
    else {
        println!("X is less than 40, mulitplying with 3");
        x * 3
    }
}
