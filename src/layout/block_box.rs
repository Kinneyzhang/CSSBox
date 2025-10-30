//! 块级盒子实现
//!
//! 对应 Java 版本的 BlockBox 类

use super::{Box, BoxType};

/// 块级盒子
///
/// 对应 Java 版本: `org.fit.cssbox.layout.BlockBox`
#[derive(Debug, Clone)]
pub struct BlockBox {
    /// 基础盒模型
    pub base: Box,
    
    /// 子盒子列表
    pub children: Vec<BlockBox>,
}

impl BlockBox {
    /// 创建新的块级盒子
    pub fn new() -> Self {
        BlockBox {
            base: Box::new(BoxType::Block),
            children: Vec::new(),
        }
    }
    
    /// 添加子盒子
    pub fn add_child(&mut self, child: BlockBox) {
        self.children.push(child);
    }
}

impl Default for BlockBox {
    fn default() -> Self {
        Self::new()
    }
}
