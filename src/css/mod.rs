//! CSS 解析和样式计算模块
//!
//! 对应 Java 版本的 `org.fit.cssbox.css` 包
//!
//! # 实现说明
//!
//! ## ⚠️ CSS 解析库差异
//!
//! Java 版本使用 jStyleParser 进行 CSS 解析，Rust 版本使用以下库：
//! - `lightningcss`: 主要的 CSS 解析和转换库
//! - `cssparser`: Mozilla 的 CSS 解析基础库
//!
//! 这些库提供了与 jStyleParser 类似的功能，但 API 设计不同。
//! 某些非标准的 CSS 扩展可能不被支持。

mod dom_analyzer;
mod css_norm;
mod font_decoder;
mod css_units;

pub use dom_analyzer::{DOMAnalyzer, Origin};
pub use css_norm::CSSNorm;
pub use font_decoder::FontDecoder;
pub use css_units::CSSUnits;
