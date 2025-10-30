//! AWT 抽象层模块
//!
//! 对应 Java 版本的 `org.fit.cssbox.awt` 包
//!
//! # ⚠️ 重要限制说明
//!
//! Java AWT (Abstract Window Toolkit) 是 Java 的原生图形框架，
//! 在 Rust 中没有直接等价物。本模块提供了一个抽象层来模拟
//! AWT 的核心功能，但使用不同的底层实现。
//!
//! ## 图形后端选择
//!
//! Rust 生态系统中有多个图形库可选：
//!
//! 1. **skia-safe**: Skia 的 Rust 绑定（Google Chrome/Android 使用）
//!    - 优点: 功能强大，性能优秀
//!    - 缺点: 编译复杂，需要 C++ 工具链
//!
//! 2. **cairo-rs**: Cairo 的 Rust 绑定（GTK+ 使用）
//!    - 优点: 成熟稳定，广泛使用
//!    - 缺点: 需要系统库
//!
//! 3. **tiny-skia**: 纯 Rust 的 2D 图形库
//!    - 优点: 无外部依赖
//!    - 缺点: 功能相对有限
//!
//! ## API 差异
//!
//! - Java AWT Graphics2D -> Rust Canvas 抽象
//! - Java Font -> Rust FontSpec + font-kit
//! - Java BufferedImage -> Rust image::ImageBuffer
//!
//! ## 不支持的特性
//!
//! 以下 Java AWT 特性在 Rust 中难以或无法实现：
//! - AWT 事件系统（需要窗口管理器集成）
//! - 系统剪贴板访问（需要平台特定代码）
//! - 打印支持（需要平台特定 API）
//! - 某些高级字体特性（如 OpenType 特性）

mod graphics_engine;
mod canvas;
mod transform;

pub use graphics_engine::GraphicsEngine;
pub use canvas::Canvas;
pub use transform::Transform;
