//! 元素背景处理
//!
//! 对应 Java 版本的 ElementBackground 类

/// 元素背景
#[derive(Debug, Clone)]
pub struct ElementBackground {
    /// 背景颜色 (RGBA)
    pub color: Option<(u8, u8, u8, u8)>,
    
    /// 背景图像 URL
    pub image_url: Option<String>,
    
    /// 背景重复
    pub repeat: BackgroundRepeat,
    
    /// 背景位置
    pub position: (f32, f32),
}

/// 背景重复方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

impl ElementBackground {
    pub fn new() -> Self {
        ElementBackground {
            color: None,
            image_url: None,
            repeat: BackgroundRepeat::Repeat,
            position: (0.0, 0.0),
        }
    }
}

impl Default for ElementBackground {
    fn default() -> Self {
        Self::new()
    }
}
