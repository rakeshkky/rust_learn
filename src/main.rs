fn main() {
    let my_tuple: (i32, f32, bool) = (5, 5.5, true);
    // Array is initialized with size. Has fixed size and equivalent to primitive types
    // Stored in a stack
    // The following is equivalent to 'let my_array = [1, 2, 3, 4];'
    let my_array: [i32; 4] = [1, 2, 3, 4];
    println!("my_tuple 2nd element is {}", my_tuple.1);
    println!("my_array 2nd element is {}", my_array[1]);
}
