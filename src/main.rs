fn main() {
    let mut x = 5;
    println!("X is {}", x);
    x = 6;
    let y = 5.5;
    print_numbers(x, y);
    let z = 7;
    let sum = add_numbers(x, z);
    println!("Sum of X and Z is {}", sum);
}

fn print_numbers(x: i32, y: f32) -> () {
    // The following is a statement ending with a ;
    // The functions is not returning anything
    println!("X is {}, Y is {}", x, y);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    // Following is expression not ending with a ;
    // Value should be returned through an expression
    x + y
}
