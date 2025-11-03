# CSSBox 架构分析 - Emacs Lisp 重新实现指南

## 概述

本仓库现在包含了一份全面的 CSSBox 项目架构分析文档，专门为使用 Emacs Lisp 重新实现该项目而编写。

## 文档位置

📄 **主要文档**: [ARCHITECTURE_ANALYSIS.md](./ARCHITECTURE_ANALYSIS.md)

## 文档内容

这份 48KB、1652 行的综合分析文档包含以下内容：

### 1. 系统架构概览
- 流水线架构：文档加载 → DOM 解析 → 样式分析 → 布局计算 → 渲染
- 设计模式：工厂模式、组合模式、策略模式、访问者模式
- 包结构和职责分析（共 27,000 行 Java 代码）

### 2. 核心组件深度剖析
- **文档加载管道** (io 包 - 453 行)
- **样式分析** (css 包 - 2,411 行)
- **布局引擎** (layout 包 - 17,195 行) ⭐ 核心部分（占 63% 代码）
- **渲染后端** (render 包 - 1,905 行)

### 3. 关键算法详解
- CSS 层叠算法（cascade algorithm）
- 盒模型布局算法（box layout）
- 选择器匹配算法（selector matching）
- **所有算法都提供了完整的 Elisp 实现示例**

### 4. 函数式编程适配
- 消除全局状态的策略
- 纯函数设计模式
- 不可变数据结构
- 状态线程化（state threading）技术

### 5. 实现路线图
分为 5 个阶段，包含时间估计和复杂度评级：
- **阶段 1**: 基础设施（2-3 周）
- **阶段 2**: CSS 基础（3-4 周）
- **阶段 3**: 盒模型（4-5 周）
- **阶段 4**: 渲染（2-3 周）
- **阶段 5**: 高级功能（持续进行）

### 6. Elisp 函数命名规范
使用 `etml-` 作为包前缀（ETML = Emacs Text Markup Language）：

```elisp
;; 公共 API
(etml-parse-html html-string)
(etml-render-url url)
(etml-css-compute-style element rules)
(etml-layout-box-tree box-tree)

;; 内部函数（双横线）
(etml--parse-selector selector)
(etml--compute-width style available-width)

;; 类型构造器
(make-etml-box ...)
(make-etml-block-box ...)

;; 类型谓词
(etml-box-p obj)
(etml-block-box-p obj)
```

### 7. 模块结构
推荐的文件组织（14+ 个模块）：
```
etml/
├── etml.el                  # 主入口点
├── etml-core.el             # 核心类型
├── etml-dom.el              # DOM 工具
├── etml-html-parser.el      # HTML 解析
├── etml-css-parser.el       # CSS 解析
├── etml-css-selector.el     # 选择器匹配
├── etml-css-cascade.el      # 层叠算法
├── etml-box.el              # 盒结构
├── etml-box-factory.el      # 盒树创建
├── etml-layout.el           # 布局入口
├── etml-layout-block.el     # 块布局
├── etml-layout-inline.el    # 内联布局
├── etml-render.el           # 渲染入口
├── etml-render-buffer.el    # 缓冲区渲染
└── etml-render-svg.el       # SVG 渲染
```

### 8. 测试策略
- 单元测试（使用 ERT）
- 视觉回归测试
- 50+ 测试用例覆盖核心功能

### 9. 主要挑战和解决方案
- CSS 解析复杂性
- 布局算法复杂性
- Emacs 中的字体度量
- 不可变性性能权衡

### 10. 实用示例模式
提供 4 种实际使用模式：
1. 渲染 URL 到缓冲区
2. 渲染 HTML 字符串
3. 自定义渲染
4. 提取结构化数据

### 11. 完整函数索引
在附录 A 中列出了所有推荐的公共函数（80+ 个函数）

## 快速开始

### 第一步：设置项目
```bash
mkdir etml
cd etml
git init
touch etml.el etml-core.el etml-dom.el
```

### 第二步：定义核心类型
```elisp
;; etml-core.el
(cl-defstruct etml-box
  node bounds abs-bounds parent children ...)
```

### 第三步：实现基础功能
从 HTML 解析器开始，然后是 CSS，最后是布局。

详细步骤请参考 [ARCHITECTURE_ANALYSIS.md](./ARCHITECTURE_ANALYSIS.md) 第 15 章。

## 预期工作量

**最小可行实现版本 (v1):**
- 核心引擎：8-10 周
- 总代码量：约 6,000-8,000 行 Elisp
- 高级功能：持续开发

## 成功标准（v1）

- [x] 解析简单 HTML 文档
- [x] 应用基础 CSS（标签、类、ID 选择器）
- [x] 计算块元素布局（不含浮动）
- [x] 渲染到 Emacs 缓冲区（使用文本属性）
- [x] 处理基本属性（width, height, margin, padding, color, font-size）
- [x] 函数式 API（无全局状态）
- [x] 50+ 测试用例

## 关键功能亮点

### 函数式设计
所有函数都是纯函数，不依赖全局变量：
```elisp
;; 纯函数 - 无副作用
(defun etml-layout-box (box available-width)
  "计算盒子布局，返回新的盒子对象。"
  (let* ((width (etml--compute-width box available-width))
         (children (etml--layout-children box width))
         (height (etml--compute-height children)))
    (etml--create-box-with-bounds box width height children)))
```

### Emacs 集成
充分利用 Emacs 内置功能：
- `libxml-parse-html-region` 用于 HTML 解析
- `url-retrieve-synchronously` 用于 HTTP 获取
- 文本属性用于样式
- SVG 支持用于精确渲染

## 与 CSSBox 的比较

| 方面 | CSSBox (Java) | ETML (Elisp) |
|------|--------------|--------------|
| **语言** | Java (面向对象) | Elisp (函数式) |
| **可变性** | 可变盒子 | 不可变（或最小可变） |
| **状态** | 全局缓存 | 通过函数线程化 |
| **继承** | 类层次结构 | 结构组合 |
| **渲染** | Java2D Graphics | Emacs 文本属性 / SVG |
| **性能** | 快速 (~100ms) | 较慢 (~1s 可接受) |
| **完整性** | 完整 CSS 2.1 + CSS3 子集 | 最初仅支持最小 CSS 子集 |

## 参考资源

- **CSSBox 手册**: `/doc/manual/manual.html`
- **CSS 2.1 规范**: https://www.w3.org/TR/CSS21/
- **W3C CSS 测试套件**: https://test.csswg.org/
- **Emacs Lisp 手册**: `(info "(elisp)")`

## 最后建议

这是一个相当大的项目（v1 需要 2-3 个月）。CSSBox 的代码库成熟且结构良好，是一个优秀的参考。函数式适配到 Elisp 是直接的，但需要严格的不可变性。从最小的可能子集开始，逐步扩展。最初优先考虑正确性而不是性能。

祝重新实现顺利！🚀

---

## 许可证

CSSBox 原项目使用 GNU Lesser General Public License v3.0 (LGPL-3.0)。
重新实现时请遵守相应的开源许可协议。

## 作者信息

- **原始项目**: CSSBox by Radek Burget
- **架构分析**: GitHub Copilot AI Assistant
- **目标实现者**: Kinneyzhang

## 联系方式

如有问题或建议，请通过 GitHub Issues 联系。

---

*文档版本: 1.0*  
*日期: 2025-11-03*  
*语言: 中文 + 英文*
