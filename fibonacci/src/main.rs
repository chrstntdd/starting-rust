use std::io;

fn main() {
    println!("How many fibonacci numbers would u like?");

    fn fibonacci(n: u64) -> u64 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let tmp = a;
            a = b;
            b = a + tmp;
            println!("current number in the sequence: {}", b);
        }

        b
    }

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let result: u64 = match user_input.trim().parse() {
        Ok(num) => fibonacci(num),
        Err(_) => 1,
    };

    println!("Here u go folks :{}", result)
}
