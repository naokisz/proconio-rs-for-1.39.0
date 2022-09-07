// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proconio::{input, is_stdin_empty};

fn test_stdin() {
    assert!(!is_stdin_empty());
    input! {
        n: usize,
    }
    assert!(!is_stdin_empty());
    println!("{}", n);

    for c in 0..n {
        println!("start {}", c);
        assert!(!is_stdin_empty());
        input! {
            i: isize,
            j: isize,
        }

        println!("{} {}", i, j);
    }
    assert!(is_stdin_empty());
}

fn test_for(input: &str, expected_stdout: &str) {
    use assert_cli::Assert;
    use std::env::args;
    Assert::command(&[&*args().next().unwrap(), "foo"])
        .stdin(input)
        .stdout()
        .is(expected_stdout)
        .and()
        .stderr()
        .is("")
        .unwrap();
}

fn main() {
    use std::env::args;
    if args().len() == 1 {
        test_for(
            "3\n1 2\n3 4\n5 6\n",
            "3\nstart 0\n1 2\nstart 1\n3 4\nstart 2\n5 6\n",
        );
        return;
    }

    test_stdin();
}
