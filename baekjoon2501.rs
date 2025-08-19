use std::io;

//만약 값이 수천만 수억 단위로 나오면서, 시간복잡도를 줄여야한다면 소수를 매핑해준 뒤에 소수만 비교

fn main(){
    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("Read Failed.");

    let mut numbers = input.split_whitespace();
    let n: u16 = numbers.next().unwrap().parse().unwrap();
    let k: usize = numbers.next().unwrap().parse().unwrap();


    //filter에 걸러지는 값들 중 k - 1 번째(Index 0부터!)
    let result = (1..=n).filter(|&i| n % i == 0).nth(k-1);

    match result{
        Some(divisor) => println!("{divisor}"),
        None => println!("0")
    }

}