use std::io;
use std::cmp::min;

macro_rules! max_cows{
    ($n:expr, $w:expr, $h:expr, $l:expr) => {
        {
            let fit_w = $w / $l;
            let fit_h = $h / $l;
            let max_fit = fit_w * fit_h;
            min($n, max_fit)
        }
    };
}


fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read failed.");

    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let w: i32 = iter.next().unwrap().parse().unwrap();
    let h: i32 = iter.next().unwrap().parse().unwrap();
    let l: i32 = iter.next().unwrap().parse().unwrap();
    let result = max_cows!(n,w,h,l);

    println!("{}",result);
}


