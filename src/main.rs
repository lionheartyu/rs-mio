mod timestamp;
mod noncopyable;
mod logger;
fn main() {
    let _ts = timestamp::Timestamp::new();
    let _ts2 = timestamp::Timestamp::from_microseconds(123456);
    let ts = timestamp::Timestamp::now();
    println!("{}", ts.to_string());

    //使用logger打印一条Info日志
    logger::Logger::instance().log(logger::LogLevel::Info, "程序启动成功！");
}