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
        n: usize,
        a: [i64;n]
    };


    let mut ans = 0i64;
    for i in 0..n {
        if a[i] > 10 {
            ans += a[i] - 10;
        }
    }

    println!("{}", ans);
}
