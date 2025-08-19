use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read Failed");
    let n: u32 = input.trim().parse().expect("Failed to parse");
    let mut result = 1;
    let mut boundary = 11;
    while n >= boundary {
        result = result + 1;
        boundary = (boundary * 10) + 1;
    }

    println!("{ }",result);
}