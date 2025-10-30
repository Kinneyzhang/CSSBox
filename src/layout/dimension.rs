//! 尺寸和坐标类型定义
//!
//! 对应 Java 版本的 Dimension 类

/// 表示二维尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

impl Dimension {
    pub fn new(width: f32, height: f32) -> Self {
        Dimension { width, height }
    }
}

/// 表示矩形区域
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle { x, y, width, height }
    }
}
