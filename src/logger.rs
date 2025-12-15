use std::sync::{Arc, Mutex, OnceLock};
use chrono::Local;
use std::io::{self, Write};

/// 日志级别定义
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Error,
    Fatal,
    Debug,
}

/// 日志器结构体
pub struct Logger {
    log_level: LogLevel,                // 当前日志级别
    output: Arc<Mutex<io::Stdout>>,     // 输出流，带互斥锁保证多线程安全
}

impl Logger {
    /// 获取全局唯一的 Logger 实例（单例模式）
    pub fn instance() -> &'static Logger {
        static INSTANCE: OnceLock<Logger> = OnceLock::new();
        INSTANCE.get_or_init(|| Logger {
            log_level: LogLevel::Info,
            output: Arc::new(Mutex::new(io::stdout())),
        })
    }

    /// 设置日志级别
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// 写日志，只有日志级别大于等于当前设置才会输出
    pub fn log(&self, level: LogLevel, msg: &str) {
        // 日志级别过滤
        if level as u8 > self.log_level as u8 {
            return;
        }
        // 日志级别字符串
        let level_str = match level {
            LogLevel::Info => "[INFO]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Fatal => "[FATAL]",
            LogLevel::Debug => "[DEBUG]",
        };
        // 获取当前时间
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        // 加锁输出，保证多线程下日志不会交叉
        let mut out = self.output.lock().unwrap();
        writeln!(out, "{}{}:{}", level_str, now, msg).unwrap();
    }
}