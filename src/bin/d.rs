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
        mut t: [usize; n]
    };

    t.sort_by_key(|&x| Reverse(x));

    let mut ng = 0;
    let mut ok = t.iter().sum::<usize>();

    while ok - ng > 1{
        let mid = (ok + ng) / 2;
        if check(n, mid, &t){
            ok = mid
        }
        else {
            ng = mid
        }
    }

    println!("{}", ok)
}

fn check(n:usize, x:usize, t: &Vec<usize>) -> bool{
    let mut dp = vec![vec![200_000; 200_000];n];

    for i in 0..n{
        if i == 0{
            dp[0][0] = t[i];
            dp[0][t[i]] = 0;
            continue
        }

        for j in 0..100_001 {
            if j >= t[i] {
                dp[i][j] = min(dp[i-1][j-t[i]], dp[i-1][j] + t[i])
            }
            else {
                dp[i][j] = dp[i-1][j] + t[i]
            }
        }
    }

    for j in 0..=x{
        if dp[n-1][j] <= x {
            return true
        }
    }

    return false
}
