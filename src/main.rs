mod timestamp;
mod noncopyable;
fn main() {
    let _ts = timestamp::Timestamp::new();
    let _ts2 = timestamp::Timestamp::from_microseconds(123456);
    let ts = timestamp::Timestamp::now();
    println!("{}", ts.to_string());
}