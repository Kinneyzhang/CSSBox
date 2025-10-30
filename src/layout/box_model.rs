//! 盒模型基础定义
//!
//! 对应 Java 版本的 Box 类

use super::dimension::Rectangle;

/// 盒子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxType {
    /// 块级盒子
    Block,
    /// 行内盒子
    Inline,
    /// 行内块盒子
    InlineBlock,
    /// 表格盒子
    Table,
    /// 表格行盒子
    TableRow,
    /// 表格单元格盒子
    TableCell,
}

/// CSS 盒模型基础结构
///
/// 对应 Java 版本: `org.fit.cssbox.layout.Box`
#[derive(Debug, Clone)]
pub struct Box {
    /// 盒子类型
    pub box_type: BoxType,
    
    /// 内容区域
    pub content: Rectangle,
    
    /// 内边距 (padding)
    pub padding: EdgeSizes,
    
    /// 边框 (border)
    pub border: EdgeSizes,
    
    /// 外边距 (margin)
    pub margin: EdgeSizes,
    
    /// 是否可见
    pub visible: bool,
}

/// 边缘尺寸（上右下左）
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeSizes {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        EdgeSizes { top, right, bottom, left }
    }
    
    pub fn uniform(size: f32) -> Self {
        EdgeSizes {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }
}

impl Box {
    /// 创建新的盒子
    pub fn new(box_type: BoxType) -> Self {
        Box {
            box_type,
            content: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            padding: EdgeSizes::default(),
            border: EdgeSizes::default(),
            margin: EdgeSizes::default(),
            visible: true,
        }
    }
    
    /// 获取盒子总宽度（包括 padding、border、margin）
    pub fn total_width(&self) -> f32 {
        self.content.width
            + self.padding.left + self.padding.right
            + self.border.left + self.border.right
            + self.margin.left + self.margin.right
    }
    
    /// 获取盒子总高度（包括 padding、border、margin）
    pub fn total_height(&self) -> f32 {
        self.content.height
            + self.padding.top + self.padding.bottom
            + self.border.top + self.border.bottom
            + self.margin.top + self.margin.bottom
    }
}
