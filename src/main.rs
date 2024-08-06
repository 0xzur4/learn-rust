use std::io;

fn main() {
    println!("Hello, world!");
}

fn kalkulator() -> Option<i32> {
    let mut input = String::new(); 
    println!("Input number: ");
    

    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error:  error number");
            return None;
        }
    };

    input = String::new();
    println!("Input number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let number2: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            return None;
        }
    };
    let result = number + number2;
    println!("Result {} + {} = {}", number, number2, result);
    Some(result)
}

#[test]
fn test_kalkulator(){
    let result = kalkulator();
    match result {
        Some(result) => println!("result kalkulasi: {}", result),
        None => println!("Error"),
    }
}

