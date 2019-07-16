// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use groupby::*;

fn main() {
    let s = "1515026263737430145";
    println!("{}", s);
    let ts: Vec<char> = s.chars().collect();
    let groups =
        ts.group_by(&|&ch| ((ch as u32) & 0xf) % 4);
    for g in groups {
        print!("{}: ", g.key);
        for e in g {
            print!("{}", e);
        }
        println!();
    }
}
