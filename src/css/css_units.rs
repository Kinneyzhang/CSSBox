//! CSS 单位转换
//!
//! 对应 Java 版本的 CSSUnits 类

/// CSS 单位转换工具
pub struct CSSUnits;

impl CSSUnits {
    /// 像素到点的转换比率 (72 DPI)
    pub const PX_TO_PT: f32 = 0.75;
    
    /// 点到像素的转换比率
    pub const PT_TO_PX: f32 = 1.333333;
    
    /// 英寸到像素 (96 DPI)
    pub const IN_TO_PX: f32 = 96.0;
    
    /// 厘米到像素
    pub const CM_TO_PX: f32 = 37.795276;
    
    /// 毫米到像素
    pub const MM_TO_PX: f32 = 3.7795276;
    
    /// 转换 CSS 长度值到像素
    pub fn to_pixels(value: f32, unit: &str, base_font_size: f32) -> f32 {
        match unit {
            "px" => value,
            "pt" => value * Self::PT_TO_PX,
            "in" => value * Self::IN_TO_PX,
            "cm" => value * Self::CM_TO_PX,
            "mm" => value * Self::MM_TO_PX,
            "em" => value * base_font_size,
            "rem" => value * 16.0, // 假设根字体大小为 16px
            "%" => value * base_font_size / 100.0,
            _ => value, // 未知单位，返回原值
        }
    }
}
