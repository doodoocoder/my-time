extern crate time;
use time::*;
fn main() {
    println!("Hello, world!");
    let start = time::now();//获取开始时间
    println!("done!start : {:?}",start.tm_hour);
}
