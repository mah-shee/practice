#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        s: String,

    }
    println!("{}", a + b + c);
    println!("{}", s);
}
