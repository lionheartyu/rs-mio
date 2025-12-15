use std::time::{SystemTime, UNIX_EPOCH}; // 导入系统时间相关类型
use chrono::{Local, TimeZone, NaiveDateTime}; // 导入 chrono 库的时间类型
use chrono::Datelike; // 日期方法
use chrono::Timelike; // 时间方法

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// 时间戳结构体，记录自 Unix 纪元以来的微秒数
pub struct Timestamp {
    microseconds_since_epoch: i64, // 距 Unix 纪元的微秒数
}

impl Timestamp {
    /// 构造一个空的 Timestamp，微秒数为 0
    pub fn new() -> Self {
        Timestamp { microseconds_since_epoch: 0 }
    }

    /// 用微秒数构造 Timestamp
    pub fn from_microseconds(microseconds: i64) -> Self {
        Timestamp { microseconds_since_epoch: microseconds }
    }

    /// 获取当前时间的 Timestamp（微秒精度）
    pub fn now() -> Self {
        // 获取当前系统时间与 Unix 纪元的时间差
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        // 计算微秒数
        let micros = duration.as_secs() as i64 * 1_000_000 + (duration.subsec_micros() as i64);
        Timestamp { microseconds_since_epoch: micros }
    }

    /// 转换为字符串，格式：YYYY/MM/DD HH:MM:SS
    pub fn to_string(&self) -> String {
        // 如果时间戳为 0，返回默认字符串
        if self.microseconds_since_epoch == 0 {
            return "0000/00/00 00:00:00".to_string();
        }
        // 计算秒数
        let secs = self.microseconds_since_epoch / 1_000_000;
        // 构造 NaiveDateTime（推荐用 from_timestamp_opt，避免弃用警告）
        let naive = NaiveDateTime::from_timestamp_opt(secs, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        // 转为本地时间
        let dt = Local.from_utc_datetime(&naive);
        // 格式化输出
        format!(
            "{:04}/{:02}/{:02} {:02}:{:02}:{:02}",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        )
    }
}

// 示例用法和测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        // 获取当前时间戳
        let ts = Timestamp::now();
        println!("当前时间戳: {}", ts.to_string());
        // 检查字符串不为空
        assert!(!ts.to_string().is_empty());
    }
}