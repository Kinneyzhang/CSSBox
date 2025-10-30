//! 变换矩阵
//!
//! 对应 Java 版本的 Transform 类

/// 2D 变换矩阵
///
/// 用于 CSS transform 属性
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// 变换矩阵 [a, b, c, d, e, f]
    /// 对应矩阵:
    /// | a  c  e |
    /// | b  d  f |
    /// | 0  0  1 |
    matrix: [f32; 6],
}

impl Transform {
    /// 创建单位矩阵
    pub fn identity() -> Self {
        Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }
    
    /// 创建平移变换
    pub fn translate(tx: f32, ty: f32) -> Self {
        Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, tx, ty],
        }
    }
    
    /// 创建缩放变换
    pub fn scale(sx: f32, sy: f32) -> Self {
        Transform {
            matrix: [sx, 0.0, 0.0, sy, 0.0, 0.0],
        }
    }
    
    /// 创建旋转变换
    pub fn rotate(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Transform {
            matrix: [cos, sin, -sin, cos, 0.0, 0.0],
        }
    }
    
    /// 组合变换
    pub fn multiply(&self, other: &Transform) -> Transform {
        let a = self.matrix;
        let b = other.matrix;
        
        Transform {
            matrix: [
                a[0] * b[0] + a[2] * b[1],
                a[1] * b[0] + a[3] * b[1],
                a[0] * b[2] + a[2] * b[3],
                a[1] * b[2] + a[3] * b[3],
                a[0] * b[4] + a[2] * b[5] + a[4],
                a[1] * b[4] + a[3] * b[5] + a[5],
            ],
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
