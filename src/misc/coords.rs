//! 坐标和几何工具
//!
//! 对应 Java 版本的 Coords 类

/// 坐标点
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Coords { x, y }
    }
    
    /// 计算到另一点的距离
    pub fn distance_to(&self, other: &Coords) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
