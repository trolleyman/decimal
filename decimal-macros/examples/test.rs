#![feature(plugin)]
#![plugin(decimal_macros)]
extern crate decimal;

fn main() {
    let a = d128!(0.1);
    let b = d128!(0.2);
    let c = d128!(0.3);
    let res = a + b;
    let eq = res == c;
    if eq {
        println!("{} + {} = {}", a, b, res);
    } else {
        println!("{} + {} = {} (expected {})", a, b, res, c);
    }
    assert!(eq);
    
    assert_eq!(d128!(0.1), d128!(1) / d128!(10));
}
