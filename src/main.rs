use std::io;

fn main() {

    println!("Enter the nth Fibonacci number:");

    let mut fib_number = String::new();
    
    io::stdin()
        .read_line(&mut fib_number)
        .expect("Failed to read line");
    let fib_number: u32 = match fib_number.trim().parse() {
        Ok(num) => num,
        Err(_) =>{ 
            println!("Invalid number!");
            return
        }
    };
    
    println!("Iterative: {}", fib1(fib_number));
    println!("Recursive: {}", fib2(fib_number));
}

fn fib1(fib_number : u32) -> u32
{
    if fib_number == 0
    {
        return 0
    }
    if fib_number == 1
    {
        return 1
    }

    let mut a = 1;
    let mut b = 1;
    let mut c = 0;

    for _i in 2..fib_number
    {
        c = a + b;
        a = b;
        b = c;
    }
    return c
}

fn fib2(fib_number : u32) -> u32
{
    if fib_number <= 1
    {
        return fib_number
    }
    return fib2(fib_number - 1) + fib2(fib_number - 2)
}