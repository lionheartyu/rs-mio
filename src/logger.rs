use std::sync::{Arc, Mutex, OnceLock}; // 引入线程安全相关类型
use chrono::Local; // 用于获取本地时间
use std::io::{self, Write}; // 用于标准输出

/// 日志级别定义
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,   // 信息
    Error,  // 错误
    Fatal,  // 致命错误
    Debug,  // 调试
}

/// 日志器结构体
pub struct Logger {
    log_level: LogLevel,                // 当前日志级别，只有大于等于该级别的日志才会输出
    output: Arc<Mutex<io::Stdout>>,     // 标准输出流，使用 Arc+Mutex 保证多线程安全
}

impl Logger {
    /// 获取全局唯一的 Logger 实例（单例模式）
    /// 使用 OnceLock 保证 Logger 只初始化一次，且线程安全
    pub fn instance() -> &'static Logger {
        // 静态变量，存放唯一的 Logger 实例
        static INSTANCE: OnceLock<Logger> = OnceLock::new();
        // 如果未初始化则创建 Logger，否则直接返回已存在的实例
        INSTANCE.get_or_init(|| Logger {
            log_level: LogLevel::Info, // 默认日志级别为 Info
            output: Arc::new(Mutex::new(io::stdout())), // 标准输出，带互斥锁
        })
    }

    /// 设置日志级别
    /// 只有大于等于该级别的日志才会被输出
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// 写日志，只有日志级别大于等于当前设置才会输出
    /// level: 本次日志的级别
    /// msg: 日志内容
    pub fn log(&self, level: LogLevel, msg: &str) {
        // 日志级别过滤，如果本次日志级别低于当前设置则直接返回
        if level as u8 > self.log_level as u8 {
            return;
        }
        // 日志级别字符串前缀
        let level_str = match level {
            LogLevel::Info => "[INFO]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Fatal => "[FATAL]",
            LogLevel::Debug => "[DEBUG]",
        };
        // 获取当前本地时间，格式化为字符串
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        // 加锁输出，保证多线程下日志不会交叉混乱
        let mut out = self.output.lock().unwrap();
        // 输出日志到标准输出，格式为：[级别][时间]:内容
        writeln!(out, "{}{}:{}", level_str, now, msg).unwrap();
    }
}