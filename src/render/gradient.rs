//! 渐变支持
//!
//! 对应 Java 版本的 Gradient, LinearGradient, RadialGradient 类

/// 渐变停止点
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// 位置 (0.0 到 1.0)
    pub position: f32,
    
    /// 颜色 (RGBA)
    pub color: (u8, u8, u8, u8),
}

/// 渐变基类
#[derive(Debug, Clone)]
pub struct Gradient {
    /// 停止点列表
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    pub fn new() -> Self {
        Gradient {
            stops: Vec::new(),
        }
    }
    
    pub fn add_stop(&mut self, stop: GradientStop) {
        self.stops.push(stop);
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new()
    }
}

/// 线性渐变
#[derive(Debug, Clone)]
pub struct LinearGradient {
    pub base: Gradient,
    pub angle: f32, // 角度（度）
}

impl LinearGradient {
    pub fn new(angle: f32) -> Self {
        LinearGradient {
            base: Gradient::new(),
            angle,
        }
    }
}

/// 径向渐变
#[derive(Debug, Clone)]
pub struct RadialGradient {
    pub base: Gradient,
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

impl RadialGradient {
    pub fn new(center_x: f32, center_y: f32, radius: f32) -> Self {
        RadialGradient {
            base: Gradient::new(),
            center_x,
            center_y,
            radius,
        }
    }
}
