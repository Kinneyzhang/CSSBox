//! CSSBox Rust Implementation
//! 
//! CSSBox 是一个用纯 Rust 实现的 HTML/CSS 渲染引擎。
//! 这是原 Java 版本 CSSBox 的完整 Rust 重新实现。
//!
//! # 主要功能
//!
//! - HTML/XML 文档解析
//! - CSS 样式表解析和应用
//! - 盒模型布局计算
//! - 文档渲染为图像
//!
//! # 架构说明
//!
//! 本项目包含以下主要模块：
//!
//! - `layout`: 布局引擎，包含盒模型和定位逻辑
//! - `css`: CSS 解析、样式计算和规范化
//! - `io`: 文档加载和 DOM 源处理
//! - `render`: 渲染引擎，包括背景、渐变等
//! - `awt`: 图形抽象层（模拟 Java AWT）
//! - `misc`: 工具函数和辅助类
//! - `testing`: 测试框架
//!
//! # 限制说明 (Rust Implementation Limitations)
//!
//! ## ⚠️ Java AWT 替代方案
//! 
//! Java 的 AWT (Abstract Window Toolkit) 在 Rust 中没有直接等价物。
//! 本实现使用以下替代方案：
//! 
//! - **图形渲染**: 使用 `skia-safe` 或 `cairo-rs` 替代 Java2D
//! - **字体处理**: 使用 `font-kit` 替代 Java Font API
//! - **图像处理**: 使用 `image` crate 替代 `java.awt.image`
//! 
//! **影响**: 
//! - 字体渲染可能与 Java 版本略有差异
//! - 需要平台特定的图形库依赖
//! - 某些高级 AWT 特性可能无法完全实现
//!
//! ## ⚠️ DOM 处理差异
//!
//! Java 使用 W3C DOM API (org.w3c.dom)，Rust 生态系统有不同的方案：
//! 
//! - **HTML 解析**: 使用 `html5ever` 替代 NekoHTML
//! - **DOM 操作**: 使用 `kuchiki` 或 `scraper` 替代 W3C DOM
//! 
//! **影响**:
//! - API 接口设计不同，但功能等价
//! - DOM 遍历和操作的性能特征可能不同
//!
//! ## ⚠️ CSS 解析库
//!
//! Java 版本使用 jStyleParser，Rust 实现使用：
//! 
//! - `lightningcss`: 高性能 CSS 解析器（最接近 jStyleParser）
//! - `cssparser`: Mozilla 的 CSS 解析库
//! 
//! **影响**:
//! - CSS 解析结果应该完全兼容 CSS3 标准
//! - 某些非标准 CSS 扩展可能不支持
//!
//! ## ⚠️ 线程模型差异
//!
//! - **Java**: 使用传统的线程和同步原语
//! - **Rust**: 使用所有权系统和 `Arc`/`Mutex` 进行线程安全
//! 
//! **影响**:
//! - Rust 版本具有编译时线程安全保证
//! - 某些设计模式需要重新思考（避免共享可变状态）
//!
//! ## ⚠️ 反射和动态特性
//!
//! Java 的反射机制在 Rust 中不可用：
//! 
//! - 使用 trait objects 和泛型替代
//! - 使用枚举和模式匹配替代类型检查
//! 
//! **影响**:
//! - 代码可能更冗长但类型安全性更强
//! - 运行时类型发现功能受限
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use cssbox_rust::layout::Engine;
//! use cssbox_rust::io::DocumentSource;
//! 
//! // 加载和解析文档
//! let doc_source = DocumentSource::from_url("http://example.com")?;
//! let dom = doc_source.parse()?;
//! 
//! // 创建渲染引擎
//! let mut engine = Engine::new(dom);
//! engine.create_layout(1200, 800)?;
//! 
//! // 渲染为图像
//! let image = engine.render_to_image()?;
//! image.save("output.png")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod layout;
pub mod css;
pub mod io;
pub mod render;
pub mod awt;
pub mod misc;
pub mod testing;

pub use layout::Engine;
pub use css::DOMAnalyzer;
pub use io::{DocumentSource, DOMSource};

/// 错误类型定义
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CSSBoxError {
        #[error("解析错误: {0}")]
        ParseError(String),
        
        #[error("布局错误: {0}")]
        LayoutError(String),
        
        #[error("渲染错误: {0}")]
        RenderError(String),
        
        #[error("IO 错误: {0}")]
        IoError(#[from] std::io::Error),
        
        #[error("网络错误: {0}")]
        NetworkError(String),
        
        #[error("未实现的特性: {0}")]
        NotImplemented(String),
    }

    pub type Result<T> = std::result::Result<T, CSSBoxError>;
}

pub use error::{CSSBoxError, Result};
