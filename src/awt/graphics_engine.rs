//! 图形引擎
//!
//! 对应 Java 版本的 GraphicsEngine 类

use crate::Result;
use crate::layout::{Engine, Dimension};
use crate::css::DOMAnalyzer;

/// 图形引擎
///
/// 使用图形后端进行渲染的引擎实现
///
/// ⚠️ 注意: 此实现需要选择一个图形后端（skia-safe 或 cairo-rs）
pub struct GraphicsEngine {
    /// 基础引擎
    base_engine: Engine,
}

impl GraphicsEngine {
    /// 创建新的图形引擎
    pub fn new(root: String, analyzer: DOMAnalyzer, base_url: String) -> Self {
        GraphicsEngine {
            base_engine: Engine::new(root, analyzer, base_url),
        }
    }
    
    /// 创建布局
    pub fn create_layout(&mut self, dim: Dimension) -> Result<()> {
        self.base_engine.create_layout(dim.width, dim.height)
    }
    
    /// 获取渲染后的图像
    ///
    /// ⚠️ 注意: 需要图形库支持
    pub fn get_image(&self) -> Result<Vec<u8>> {
        Err(crate::CSSBoxError::NotImplemented(
            "图像生成需要启用图形后端 (skia-safe 或 cairo)".to_string()
        ))
    }
}
