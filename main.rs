fn main() {
    let mut input = String::new();

    println!("Please input two integers, separated by a space:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let v: Vec<&str> = input.trim().split(" ").collect();

            let a: i32 = match v[0].parse() {
                Ok(num) => num,
                Err(_) => panic!("Please input a valid integer!")
            };

            let b: i32 = match v[1].parse() {
                Ok(num) => num,
                Err(_) => panic!("Please input a valid integer!")
            };

            let result = a.checked_add(b);

            match result {
                Some(x) => println!("{} + {} = {}", a, b, x),
                None => println!("Overflow!"),
            }
        },
        Err(_) => {
            println!("Please input a valid integer!");
        },
    }
}