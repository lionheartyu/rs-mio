// 一个空结构体,用于标记不可复制类型
pub struct NonCopyable {
    // 私有字段,防止外部构造,仅用于类型标记
    _private: (),
}

impl NonCopyable {
    /// 创建一个新的 NonCopyable 实例
    pub fn new() -> Self {
        NonCopyable { _private: () }
    }
}