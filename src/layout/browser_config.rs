//! 浏览器配置
//!
//! 对应 Java 版本的 BrowserConfig 类

use std::collections::HashMap;

/// 浏览器配置选项
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// 是否加载图像
    pub load_images: bool,
    
    /// 是否加载背景图像
    pub load_background_images: bool,
    
    /// 是否裁剪视口
    pub clip_viewport: bool,
    
    /// 逻辑字体映射
    logical_fonts: HashMap<String, Vec<String>>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        let mut config = BrowserConfig {
            load_images: true,
            load_background_images: true,
            clip_viewport: false,
            logical_fonts: HashMap::new(),
        };
        
        // 设置默认字体映射
        config.set_logical_font("serif", vec!["Times New Roman", "Times", "Georgia"]);
        config.set_logical_font("sans-serif", vec!["Arial", "Helvetica", "Verdana"]);
        config.set_logical_font("monospace", vec!["Courier New", "Courier", "Monaco"]);
        
        config
    }
}

impl BrowserConfig {
    /// 设置逻辑字体系列的字体列表
    pub fn set_logical_font(&mut self, family: &str, fonts: Vec<&str>) {
        self.logical_fonts.insert(
            family.to_string(), 
            fonts.iter().map(|s| s.to_string()).collect()
        );
    }
    
    /// 获取逻辑字体系列对应的字体列表
    pub fn get_logical_font(&self, family: &str) -> Option<&Vec<String>> {
        self.logical_fonts.get(family)
    }
}
