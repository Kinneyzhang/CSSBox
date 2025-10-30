//! 渲染模块
//!
//! 对应 Java 版本的 `org.fit.cssbox.render` 包
//!
//! 负责将布局后的盒模型渲染为可视化输出

mod box_renderer;
mod gradient;
mod background;

pub use box_renderer::BoxRenderer;
pub use gradient::{Gradient, LinearGradient, RadialGradient, GradientStop};
pub use background::ElementBackground;
