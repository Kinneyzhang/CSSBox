//! 行内盒子实现
//!
//! 对应 Java 版本的 Inline 和 InlineBox 类

use super::{Box, BoxType};

/// 行内盒子
///
/// 对应 Java 版本: `org.fit.cssbox.layout.InlineBox`
#[derive(Debug, Clone)]
pub struct InlineBox {
    /// 基础盒模型
    pub base: Box,
    
    /// 文本内容
    pub text: Option<String>,
}

impl InlineBox {
    /// 创建新的行内盒子
    pub fn new() -> Self {
        InlineBox {
            base: Box::new(BoxType::Inline),
            text: None,
        }
    }
    
    /// 创建带文本的行内盒子
    pub fn with_text(text: String) -> Self {
        InlineBox {
            base: Box::new(BoxType::Inline),
            text: Some(text),
        }
    }
}

impl Default for InlineBox {
    fn default() -> Self {
        Self::new()
    }
}
