//! 视口管理
//!
//! 对应 Java 版本的 Viewport 类

use super::Dimension;

/// 视口
///
/// 表示浏览器的可视区域
#[derive(Debug, Clone)]
pub struct Viewport {
    /// 视口尺寸
    pub size: Dimension,
    
    /// 滚动位置
    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl Viewport {
    /// 创建新视口
    pub fn new(size: Dimension) -> Self {
        Viewport {
            size,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }
    
    /// 设置滚动位置
    pub fn scroll_to(&mut self, x: f32, y: f32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }
}
