use std::io;

fn main() {
    println!("How many fibonacci numbers would u like?");

    fn fibonacci(n: i64) -> i64 {
        if n < 0 {
            println!("No can do, nothing less than zero can be used. ");
            0
        } else if n == 1 {
            0
        } else if n == 2 {
            1
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let result: i64 = match user_input.trim().parse() {
        Ok(num) => fibonacci(num),
        Err(_) => 1,
    };

    println!("Here u go folks :{}", result)
}
