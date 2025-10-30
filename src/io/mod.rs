//! IO 和文档源处理模块
//!
//! 对应 Java 版本的 `org.fit.cssbox.io` 包
//!
//! # 实现说明
//!
//! ## ⚠️ HTML 解析差异
//!
//! Java 版本使用 NekoHTML 进行 HTML 解析，Rust 版本使用：
//! - `html5ever`: Servo 的 HTML5 解析器
//! - `kuchiki`: 基于 html5ever 的 DOM 操作库
//!
//! html5ever 是一个完全符合 HTML5 规范的解析器，但 API 与
//! NekoHTML 完全不同。

mod document_source;
mod dom_source;

pub use document_source::DocumentSource;
pub use dom_source::DOMSource;
