use std::io;

fn main() {
    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("Read Failed.");

    //중요중요!! mutable이여야 빌려줄 수 있어!!!
    let mut numbers = input.split_whitespace();
    
    //numbers.0과 numbers.1을 꺼내서- 얘를 parse를 통해 u8로 변환해준 뒤에 또 unwrap. 
    let a: u8 = numbers.next().unwrap().parse().unwrap();
    let b: u8 = numbers.next().unwrap().parse().unwrap();
    
    let r = a - b;

    println!("{}",r);
}