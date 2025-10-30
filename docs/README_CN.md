# CSSBox Rust 实现文档

## 项目概述

CSSBox Rust 是 Java 版本 CSSBox 的完整重新实现。CSSBox 是一个 HTML/CSS 渲染引擎，能够解析 HTML 文档、应用 CSS 样式、计算布局并渲染为图像。

### 原始 Java 项目信息

- **项目名称**: CSSBox
- **作者**: Radek Burget (burgetr@fit.vutbr.cz)
- **许可证**: GNU Lesser General Public License 3.0
- **官网**: http://cssbox.sourceforge.net/
- **代码规模**: 91 个 Java 文件，约 27,000 行代码

### Rust 重新实现目标

本项目的目标是：
1. 使用 Rust 完全重新实现 CSSBox 的核心功能
2. 保持与原版相似的 API 设计（在 Rust 语言特性允许的范围内）
3. 利用 Rust 生态系统中的优秀库
4. 提供更好的内存安全性和并发性能

## 架构说明

### 模块结构

项目分为以下主要模块：

```
src/
├── lib.rs          # 库入口和公共 API
├── layout/         # 布局引擎（盒模型、定位）
│   ├── engine.rs
│   ├── box_model.rs
│   ├── block_box.rs
│   ├── inline_box.rs
│   ├── viewport.rs
│   └── browser_config.rs
├── css/            # CSS 解析和样式计算
│   ├── dom_analyzer.rs
│   ├── css_norm.rs
│   ├── font_decoder.rs
│   └── css_units.rs
├── io/             # 文档加载和 DOM 处理
│   ├── document_source.rs
│   └── dom_source.rs
├── render/         # 渲染引擎
│   ├── box_renderer.rs
│   ├── gradient.rs
│   └── background.rs
├── awt/            # 图形抽象层（模拟 Java AWT）
│   ├── graphics_engine.rs
│   ├── canvas.rs
│   └── transform.rs
├── misc/           # 工具函数
│   ├── base64.rs
│   └── coords.rs
└── testing/        # 测试框架
    └── image_comparator.rs
```

### Java 到 Rust 映射

| Java 包 | Rust 模块 | 说明 |
|---------|----------|------|
| `org.fit.cssbox.layout` | `layout` | 布局引擎 |
| `org.fit.cssbox.css` | `css` | CSS 处理 |
| `org.fit.cssbox.io` | `io` | IO 和文档源 |
| `org.fit.cssbox.render` | `render` | 渲染引擎 |
| `org.fit.cssbox.awt` | `awt` | 图形抽象层 |
| `org.fit.cssbox.misc` | `misc` | 工具函数 |
| `org.fit.cssbox.testing` | `testing` | 测试工具 |

## 依赖库说明

### HTML/XML 解析

**Java 版本**: 
- NekoHTML (HTML 解析)
- Xerces 2 (XML 处理)

**Rust 版本**:
- `html5ever`: Servo 项目的 HTML5 解析器
- `kuchiki`: 基于 html5ever 的 DOM 操作库
- `scraper`: HTML 解析和查询

**差异说明**:
- html5ever 完全符合 HTML5 规范
- API 设计完全不同，但功能等价
- 性能可能更好（Rust 的零成本抽象）

### CSS 解析

**Java 版本**:
- jStyleParser 4.0.1

**Rust 版本**:
- `lightningcss`: 高性能 CSS 解析器（由 Parcel 开发）
- `cssparser`: Mozilla 的 CSS 解析基础库

**差异说明**:
- lightningcss 支持最新的 CSS 规范
- 解析速度通常比 Java 版本更快
- 某些非标准 CSS 扩展可能不支持

### 图形和渲染

**Java 版本**:
- Java AWT (Abstract Window Toolkit)
- Java2D

**Rust 版本（可选其一）**:
- `skia-safe`: Skia 的 Rust 绑定（推荐）
- `cairo-rs`: Cairo 的 Rust 绑定
- `tiny-skia`: 纯 Rust 2D 图形库

**差异说明**:
这是最大的差异点。详见下文"实现限制"部分。

### 字体处理

**Java 版本**:
- java.awt.Font
- java.awt.FontMetrics

**Rust 版本**:
- `font-kit`: 跨平台字体查询和加载
- `pathfinder_geometry`: 字体几何处理

**差异说明**:
- 字体查找和加载方式不同
- 字形渲染可能有细微差异
- 某些高级 OpenType 特性可能不支持

### 网络和 HTTP

**Java 版本**:
- java.net.URL
- java.net.HttpURLConnection

**Rust 版本**:
- `reqwest`: HTTP 客户端（可选 feature）
- `url`: URL 解析和处理

## 实现限制和差异

### ⚠️ 1. Java AWT 无法完全替代

**问题描述**:
Java AWT 是一个完整的窗口工具包，包含：
- 窗口管理
- 事件处理系统
- 2D 图形渲染
- 字体系统
- 图像处理
- 打印支持

**Rust 解决方案**:
我们只实现了 2D 图形渲染部分，使用以下库：
- 图形: skia-safe 或 cairo-rs
- 字体: font-kit
- 图像: image crate

**无法实现的功能**:
- ❌ AWT 事件系统（需要窗口管理器）
- ❌ 系统剪贴板访问
- ❌ 打印支持
- ❌ 某些平台特定的字体特性

**影响**:
- 只能离线渲染为图像，不能显示交互式窗口
- 某些字体渲染效果可能与 Java 版本略有差异

### ⚠️ 2. DOM API 差异

**问题描述**:
Java 使用 W3C DOM API (org.w3c.dom)，这是一个标准但较旧的 API。

**Rust 解决方案**:
使用 html5ever + kuchiki，它们提供更现代的 API：
```rust
// Java 风格
Element element = doc.getElementById("id");

// Rust 风格  
let element = document.select("#id").unwrap();
```

**差异**:
- API 设计完全不同
- Rust 版本更符合现代 Web 标准
- 某些 DOM 操作可能需要不同的方法

### ⚠️ 3. 反射和运行时类型检查

**问题描述**:
Java 广泛使用反射机制：
```java
if (obj instanceof BlockBox) {
    BlockBox box = (BlockBox) obj;
    // ...
}
```

**Rust 解决方案**:
使用枚举和模式匹配：
```rust
match box_type {
    BoxType::Block(block_box) => {
        // ...
    }
    BoxType::Inline(inline_box) => {
        // ...
    }
}
```

**影响**:
- 代码可能更冗长
- 但类型安全性更强
- 编译时检查，运行时零成本

### ⚠️ 4. 线程和并发模型

**Java 版本**:
- 使用传统线程和同步原语
- 可以随意共享可变状态

**Rust 版本**:
- 使用所有权系统
- 需要显式使用 Arc/Mutex 共享数据

**优势**:
- ✅ 编译时保证线程安全
- ✅ 没有数据竞争
- ✅ 更好的并发性能

**挑战**:
- 某些设计模式需要重新思考
- 学习曲线较陡

### ⚠️ 5. 异常处理

**Java 版本**:
```java
try {
    parseDocument();
} catch (IOException e) {
    // 处理异常
}
```

**Rust 版本**:
```rust
match parse_document() {
    Ok(doc) => { /* 使用 doc */ }
    Err(e) => { /* 处理错误 */ }
}
// 或使用 ? 操作符
let doc = parse_document()?;
```

**差异**:
- Rust 使用 Result 类型而不是异常
- 必须显式处理所有错误情况
- 更安全但代码可能更冗长

### ⚠️ 6. 平台依赖

**Java 版本**:
- 真正的跨平台（Write Once, Run Anywhere）
- JVM 处理所有平台差异

**Rust 版本**:
- 需要针对每个平台编译
- 某些依赖（如 skia-safe）需要平台特定的编译环境
- 不同平台可能需要不同的依赖

**编译要求**:
- Linux: 可能需要 libfontconfig、libfreetype
- macOS: 需要 Xcode 命令行工具
- Windows: 需要 MSVC 或 MinGW

## 使用示例

### 基本使用

```rust
use cssbox_rust::{
    io::DocumentSource,
    css::{DOMAnalyzer, CSSNorm},
    layout::Engine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载文档
    let mut doc_source = DocumentSource::from_url("http://example.com")?;
    let html = doc_source.read()?;
    
    // 2. 创建 DOM 分析器
    let mut analyzer = DOMAnalyzer::new(doc_source.get_url().to_string());
    analyzer.add_stylesheet(
        cssbox_rust::css::Origin::Agent,
        CSSNorm::std_stylesheet()
    );
    analyzer.get_stylesheets()?;
    
    // 3. 创建渲染引擎
    let mut engine = Engine::new(
        analyzer.get_root(),
        analyzer,
        doc_source.get_url().to_string()
    );
    
    // 4. 计算布局
    engine.create_layout(1200.0, 800.0)?;
    
    // 5. 渲染（需要图形库支持）
    // engine.render_to_image()?;
    
    Ok(())
}
```

### 从文件加载

```rust
use cssbox_rust::io::DocumentSource;

fn load_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let doc_source = DocumentSource::from_url("file:///path/to/file.html")?;
    // ... 继续处理
    Ok(())
}
```

### 从字符串加载

```rust
use cssbox_rust::io::DocumentSource;

fn load_from_string() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test</title></head>
        <body><h1>Hello</h1></body>
        </html>
    "#;
    
    let doc_source = DocumentSource::from_string(
        html.to_string(),
        "http://localhost/"
    );
    // ... 继续处理
    Ok(())
}
```

## 编译和运行

### 基本编译

```bash
cargo build --release
```

### 启用图形支持

```bash
# 使用 skia-safe
cargo build --release --features graphics

# 或在 Cargo.toml 中启用
[features]
default = ["graphics"]
graphics = ["skia-safe"]
```

### 运行测试

```bash
cargo test
```

### 运行示例

```bash
cargo run --example render_html
```

## 性能对比

### 预期性能特征

| 指标 | Java 版本 | Rust 版本 | 说明 |
|------|----------|----------|------|
| 解析速度 | 基准 | ~1.5-2x 快 | html5ever 性能优异 |
| 布局计算 | 基准 | ~1.2-1.5x 快 | 零成本抽象 |
| 内存使用 | 基准 | ~0.6-0.8x | 无 GC 开销 |
| 启动时间 | 慢 | 快 | 无 JVM 启动 |
| 渲染速度 | 基准 | 取决于后端 | 使用 Skia 时相近 |

### 内存管理

**Java 版本**:
- 垃圾回收（GC）
- 可能有 GC 停顿
- 内存使用较高

**Rust 版本**:
- 编译时内存管理
- 无 GC 停顿
- 更可预测的性能
- 更低的内存占用

## 未来工作

### 短期目标

- [ ] 完成 CSS 解析器集成
- [ ] 实现完整的盒模型布局算法
- [ ] 集成图形渲染后端
- [ ] 添加全面的单元测试

### 中期目标

- [ ] 支持更多 CSS3 特性
- [ ] 实现表格布局
- [ ] 支持浮动和定位
- [ ] SVG 渲染支持

### 长期目标

- [ ] 性能优化（并行布局计算）
- [ ] WASM 支持（在浏览器中运行）
- [ ] 流式渲染
- [ ] GPU 加速渲染

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

### 代码规范

- 遵循 Rust 标准代码风格
- 运行 `cargo fmt` 格式化代码
- 运行 `cargo clippy` 检查代码质量
- 为新功能添加测试

## 许可证

本项目采用 GNU Lesser General Public License 3.0 许可证，与原 Java 版本保持一致。

## 联系方式

- 原作者: Radek Burget (burgetr@fit.vutbr.cz)
- Rust 实现: [请在 GitHub 上提交 Issue]

## 致谢

- 感谢 Radek Burget 创建了优秀的 CSSBox 项目
- 感谢 Rust 社区提供的优秀库
- 感谢 Servo 项目的 html5ever 和相关工具

## 附录：完整的限制清单

### 已标注的限制

1. **Java AWT 图形系统** (src/awt/mod.rs, src/lib.rs)
   - 使用 skia-safe 或 cairo-rs 替代
   - 无法实现完整的窗口系统功能

2. **DOM API 差异** (src/io/mod.rs, src/lib.rs)
   - 使用 html5ever/kuchiki 替代 W3C DOM
   - API 设计不同但功能等价

3. **CSS 解析库** (src/css/mod.rs, src/lib.rs)
   - 使用 lightningcss 替代 jStyleParser
   - 某些非标准扩展可能不支持

4. **字体处理** (src/css/font_decoder.rs)
   - 使用 font-kit 替代 java.awt.Font
   - 某些字体特性可能不同

5. **反射机制** (src/lib.rs)
   - 使用枚举和 trait objects 替代
   - 运行时类型发现受限

6. **线程模型** (src/lib.rs)
   - 使用所有权系统替代传统锁
   - 设计模式需要调整

7. **Base64 编解码** (src/misc/base64.rs)
   - 建议使用现有的 base64 crate

8. **网络功能** (src/io/document_source.rs)
   - 需要编译时启用 network feature

9. **图像渲染** (src/layout/engine.rs, src/render/box_renderer.rs)
   - 需要图形库支持
   - 标注为 NotImplemented 直到集成图形后端

### 功能状态

| 功能 | Java 版本 | Rust 实现状态 | 备注 |
|------|----------|--------------|------|
| HTML 解析 | ✅ | ✅ | 使用 html5ever |
| CSS 解析 | ✅ | 🚧 | 框架完成，需集成 lightningcss |
| 盒模型 | ✅ | 🚧 | 基础结构完成 |
| 布局计算 | ✅ | ⏳ | 待实现 |
| 文本渲染 | ✅ | ⏳ | 待实现 |
| 图像加载 | ✅ | ⏳ | 待实现 |
| 背景渲染 | ✅ | 🚧 | 结构完成 |
| CSS 渐变 | ✅ | 🚧 | 结构完成 |
| CSS Transform | ✅ | 🚧 | 基础完成 |
| 表格布局 | ✅ | ⏳ | 待实现 |
| 浮动 | ✅ | ⏳ | 待实现 |
| 定位 | ✅ | ⏳ | 待实现 |

图例：
- ✅ 已完成
- 🚧 进行中
- ⏳ 待实现
- ❌ 不支持
