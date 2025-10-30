//! 布局引擎模块
//!
//! 该模块实现了 CSS 盒模型和布局算法。
//! 对应 Java 版本的 `org.fit.cssbox.layout` 包。
//!
//! # 主要组件
//!
//! - `Engine`: 主渲染引擎
//! - `Box`: 基础盒模型
//! - `BlockBox`: 块级盒子
//! - `InlineBox`: 行内盒子
//! - `Viewport`: 视口管理

mod engine;
mod dimension;
mod box_model;
mod block_box;
mod inline_box;
mod viewport;
mod browser_config;

pub use engine::Engine;
pub use dimension::{Dimension, Rectangle};
pub use box_model::{Box, BoxType, EdgeSizes};
pub use block_box::BlockBox;
pub use inline_box::InlineBox;
pub use viewport::Viewport;
pub use browser_config::BrowserConfig;
