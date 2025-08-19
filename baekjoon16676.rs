/*0 부터 9까지 필요한 카드 총합 0이면 1개 1이면 2개 .. 9면 10개 10 10개 11 11개 - 22 12개 33 13개 44 14개 .. 99 19개 100 20개 
111 21개 ... 999 29개 1000 30개 1111이면 31개
스티커 팩수로 계산했을 때 11이상 2개 111이상 3개 1111이상 4개 ... */

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
