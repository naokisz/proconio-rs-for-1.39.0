// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::fastout;

#[fastout]
fn foo() -> i32 {
    println!("4");
    3
}

#[allow(clippy::print_literal)]
#[fastout]
fn main() {
    let clo = || "AtCoder"; // OK;
    let name = clo();
    // let _err = || println!("hello"); // error even it's not spown
    // std::thread::spawn(|| {
    //     std::println!("hello");
    // }); // of course error
    println!();
    println!("hello, world, {}!", name);
    println!("{}", foo());
    print!("{}{}, ", 'h', "ello"); // "hello"       (no newline)
    println!("{}!", "world"); // "world!\n"
    println!("{}", 123_456_789); // "123456789\n"
}
