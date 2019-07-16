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
