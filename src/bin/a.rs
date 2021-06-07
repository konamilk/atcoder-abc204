use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        mut xy: [i64; 2]
    };

    xy.sort();

    if xy[0] == xy[1]{
        println!("{}", xy[0])
    }
    else if xy[0] == 0 && xy[1] == 1{
        println!("2");
    }
    else if xy[0] == 0 && xy[1] == 2{
        println!("1");
    }
    else {
        println!("0")
    }
}
