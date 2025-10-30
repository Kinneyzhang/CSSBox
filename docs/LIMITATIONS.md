# CSSBox Rust 实现限制清单

本文档详细列出了 Rust 版本相对于原 Java 版本的所有实现限制和差异。

## 限制概览

| # | 限制分类 | 严重程度 | 是否可解决 | 标注位置 |
|---|----------|---------|----------|---------|
| 1 | Java AWT 图形系统 | 高 | 部分 | src/awt/mod.rs, src/lib.rs |
| 2 | DOM API 差异 | 中 | 是 | src/io/mod.rs, src/lib.rs |
| 3 | CSS 解析器差异 | 低 | 是 | src/css/mod.rs, src/lib.rs |
| 4 | 字体处理差异 | 中 | 部分 | src/css/font_decoder.rs |
| 5 | 反射机制 | 低 | 是 | src/lib.rs |
| 6 | 线程模型差异 | 低 | 是 | src/lib.rs |
| 7 | 网络功能 | 低 | 是 | src/io/document_source.rs |
| 8 | Base64 编解码 | 低 | 是 | src/misc/base64.rs |
| 9 | 图像渲染 | 高 | 是 | src/layout/engine.rs |

## 详细说明

### 1. Java AWT 图形系统 ⚠️⚠️⚠️

**问题描述:**
Java AWT (Abstract Window Toolkit) 是一个完整的窗口工具包，包含窗口管理、事件系统、2D 图形、字体、图像处理等功能。Rust 没有直接等价的库。

**Java 实现:**
```java
// Java 版本
import java.awt.*;
import java.awt.image.BufferedImage;

Graphics2D g2d = image.createGraphics();
g2d.drawString("Hello", 10, 10);
g2d.setFont(new Font("Arial", Font.PLAIN, 12));
```

**Rust 替代方案:**
```rust
// Rust 版本（使用 skia-safe）
use skia_safe::{Canvas, Paint, Font};

let mut canvas = Canvas::new(...);
let paint = Paint::default();
let font = Font::from_typeface(...);
canvas.draw_str("Hello", (10, 10), &font, &paint);
```

**影响:**
- ✅ 可以实现: 2D 图形绘制、基本字体渲染、图像处理
- ❌ 无法实现: 窗口管理、AWT 事件系统、系统剪贴板、打印支持
- ⚠️ 可能不同: 字体渲染效果、抗锯齿算法

**标注位置:**
- `src/awt/mod.rs`: 模块级文档
- `src/lib.rs`: 库级文档
- `src/awt/graphics_engine.rs`: 函数级标注

**解决方案:**
1. 使用 `skia-safe` (Google Skia 的 Rust 绑定) - 推荐
2. 使用 `cairo-rs` (Cairo 图形库的 Rust 绑定)
3. 使用 `tiny-skia` (纯 Rust 实现，功能有限)

**编译要求:**
- Linux: 需要 `libfontconfig-dev`, `libfreetype6-dev`
- macOS: 需要 Xcode 命令行工具
- Windows: 需要 MSVC 或 MinGW

---

### 2. DOM API 差异 ⚠️⚠️

**问题描述:**
Java 使用 W3C DOM API (org.w3c.dom)，这是一个标准但较旧的 API。Rust 生态系统有不同的 DOM 实现。

**Java 实现:**
```java
// Java 版本
import org.w3c.dom.*;

Document doc = parser.parse(input);
Element root = doc.getDocumentElement();
NodeList nodes = root.getElementsByTagName("div");
```

**Rust 替代方案:**
```rust
// Rust 版本（使用 kuchiki）
use kuchiki::parse_html;

let document = parse_html().one(html_string);
let divs = document.select("div").unwrap();
```

**影响:**
- ✅ 功能等价，都能完成 DOM 操作
- ⚠️ API 设计完全不同，需要重新学习
- ✅ Rust 版本更符合现代 Web 标准
- ✅ 性能通常更好

**标注位置:**
- `src/io/mod.rs`: 模块级文档
- `src/io/dom_source.rs`: 实现级标注
- `src/lib.rs`: 库级文档

**解决方案:**
使用 `html5ever` + `kuchiki` 组合，提供完整的 HTML5 解析和 DOM 操作能力。

---

### 3. CSS 解析器差异 ⚠️

**问题描述:**
Java 版本使用 jStyleParser，Rust 使用 lightningcss 或 cssparser。

**Java 实现:**
```java
// Java 版本
import cz.vutbr.web.css.*;

StyleSheet sheet = CSSFactory.parseString(css, baseURL);
```

**Rust 替代方案:**
```rust
// Rust 版本（使用 lightningcss）
use lightningcss::stylesheet::StyleSheet;

let stylesheet = StyleSheet::parse(css, options)?;
```

**影响:**
- ✅ 支持完整的 CSS3 规范
- ✅ 解析速度通常更快
- ⚠️ 某些非标准 CSS 扩展可能不支持
- ✅ 更好的错误处理

**标注位置:**
- `src/css/mod.rs`: 模块级文档
- `src/css/dom_analyzer.rs`: 实现标注

**解决方案:**
使用 `lightningcss` 作为主要 CSS 解析器。

---

### 4. 字体处理差异 ⚠️⚠️

**问题描述:**
Java 的字体 API 与操作系统深度集成，Rust 需要使用独立的字体库。

**Java 实现:**
```java
// Java 版本
Font font = new Font("Arial", Font.PLAIN, 12);
FontMetrics metrics = graphics.getFontMetrics(font);
```

**Rust 替代方案:**
```rust
// Rust 版本（使用 font-kit）
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;

let font = SystemSource::new()
    .select_best_match(&[FamilyName::SansSerif], &Properties::new())?;
```

**影响:**
- ✅ 可以加载和使用系统字体
- ⚠️ 字体回退机制可能不同
- ⚠️ 字形渲染可能有细微差异
- ❌ 某些高级 OpenType 特性可能不支持

**标注位置:**
- `src/css/font_decoder.rs`: 函数级标注
- `src/lib.rs`: 库级文档

**解决方案:**
使用 `font-kit` + `pathfinder_geometry` 组合。

**系统依赖:**
- Linux: 需要 `fontconfig`
- macOS: 使用 Core Text
- Windows: 使用 DirectWrite

---

### 5. 反射机制 ⚠️

**问题描述:**
Java 的反射机制在 Rust 中不可用。

**Java 实现:**
```java
// Java 版本
if (obj instanceof BlockBox) {
    BlockBox box = (BlockBox) obj;
    // ...
}

Class<?> clazz = obj.getClass();
Method method = clazz.getMethod("render");
```

**Rust 替代方案:**
```rust
// Rust 版本
match box {
    BoxType::Block(block_box) => {
        // ...
    }
    BoxType::Inline(inline_box) => {
        // ...
    }
}
```

**影响:**
- ✅ 使用枚举和模式匹配更加类型安全
- ✅ 编译时检查，运行时零成本
- ⚠️ 代码可能更冗长
- ❌ 无法运行时动态发现类型

**标注位置:**
- `src/lib.rs`: 库级文档

**解决方案:**
1. 使用枚举代替继承层次
2. 使用 trait objects 实现多态
3. 使用泛型提供静态多态

---

### 6. 线程模型差异 ⚠️

**问题描述:**
Java 和 Rust 有不同的并发模型。

**Java 实现:**
```java
// Java 版本
public synchronized void update() {
    this.data = newData;
}
```

**Rust 替代方案:**
```rust
// Rust 版本
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(initial_data));
// 在线程间共享
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    let mut data = data_clone.lock().unwrap();
    *data = new_data;
});
```

**影响:**
- ✅ 编译时保证线程安全（无数据竞争）
- ✅ 更好的并发性能
- ⚠️ 需要显式管理共享状态
- ⚠️ 某些设计模式需要重新思考

**标注位置:**
- `src/lib.rs`: 库级文档

**优势:**
Rust 的所有权系统在编译时防止数据竞争，这比 Java 的运行时检查更安全。

---

### 7. 网络功能 ⚠️

**问题描述:**
网络功能在 Rust 中需要额外的依赖。

**Java 实现:**
```java
// Java 版本
URL url = new URL("http://example.com");
InputStream in = url.openStream();
```

**Rust 替代方案:**
```rust
// Rust 版本
use reqwest::blocking;

let response = blocking::get("http://example.com")?;
let content = response.text()?;
```

**影响:**
- ✅ 功能完全等价
- ⚠️ 需要编译时启用 feature
- ✅ 更好的异步支持

**标注位置:**
- `src/io/document_source.rs`: 函数级标注

**解决方案:**
使用 `reqwest` crate，在 Cargo.toml 中添加为可选依赖。

---

### 8. Base64 编解码 ⚠️

**问题描述:**
不需要自己实现 Base64，使用现有的 crate。

**Java 实现:**
```java
// Java 版本（自己实现）
byte[] decoded = Base64Coder.decode(input);
```

**Rust 替代方案:**
```rust
// Rust 版本
use base64::{Engine as _, engine::general_purpose};

let decoded = general_purpose::STANDARD.decode(input)?;
```

**影响:**
- ✅ 使用标准库更安全
- ✅ 性能通常更好
- ✅ 更好的错误处理

**标注位置:**
- `src/misc/base64.rs`: 函数级标注

**解决方案:**
使用 `base64` crate。

---

### 9. 图像渲染 ⚠️⚠️⚠️

**问题描述:**
图像渲染需要图形库支持。

**Java 实现:**
```java
// Java 版本
BufferedImage image = new BufferedImage(width, height, TYPE_INT_ARGB);
Graphics2D g2d = image.createGraphics();
renderContent(g2d);
ImageIO.write(image, "PNG", file);
```

**Rust 替代方案:**
```rust
// Rust 版本
use image::{RgbaImage, ImageBuffer};

let mut img = ImageBuffer::new(width, height);
// 需要图形库来绘制
// 使用 skia-safe 或 cairo
```

**影响:**
- ✅ 可以生成图像
- ⚠️ 需要选择图形后端
- ⚠️ 编译依赖较复杂

**标注位置:**
- `src/layout/engine.rs`: 函数级标注
- `src/render/box_renderer.rs`: 函数级标注

**解决方案:**
1. 集成 skia-safe（推荐）
2. 集成 cairo-rs
3. 使用 tiny-skia（纯 Rust）

---

## 功能实现状态

| 功能模块 | Java 版本 | Rust 实现 | 限制 | 备注 |
|---------|----------|----------|------|------|
| HTML 解析 | ✅ NekoHTML | ✅ html5ever | 无 | API 不同 |
| CSS 解析 | ✅ jStyleParser | ✅ lightningcss | 无 | API 不同 |
| DOM 操作 | ✅ W3C DOM | ✅ kuchiki | 无 | API 不同 |
| 盒模型 | ✅ | 🚧 | 无 | 基础完成 |
| 布局计算 | ✅ | ⏳ | 无 | 待实现 |
| 文本渲染 | ✅ | ⏳ | 字体差异 | 需要 font-kit |
| 图像加载 | ✅ | ⏳ | 无 | 使用 image crate |
| 背景渲染 | ✅ | 🚧 | 无 | 结构完成 |
| CSS 渐变 | ✅ | 🚧 | 无 | 结构完成 |
| CSS Transform | ✅ | 🚧 | 无 | 基础完成 |
| 表格布局 | ✅ | ⏳ | 无 | 待实现 |
| 浮动 | ✅ | ⏳ | 无 | 待实现 |
| 定位 | ✅ | ⏳ | 无 | 待实现 |
| 窗口显示 | ✅ | ❌ | AWT | 不支持 |
| 事件系统 | ✅ | ❌ | AWT | 不支持 |
| 剪贴板 | ✅ | ❌ | 平台特定 | 不支持 |
| 打印 | ✅ | ❌ | 平台特定 | 不支持 |

图例:
- ✅ 已完成
- 🚧 进行中
- ⏳ 待实现
- ❌ 不支持/无法实现

---

## 平台兼容性

### Linux

**优势:**
- 大部分库都有良好支持
- 开发环境配置简单

**要求:**
```bash
sudo apt-get install libfontconfig1-dev libfreetype6-dev
```

**限制:**
- 需要安装系统图形库

### macOS

**优势:**
- 系统集成度高
- Core Text 字体支持好

**要求:**
```bash
xcode-select --install
```

**限制:**
- 可能需要较新的 macOS 版本

### Windows

**优势:**
- DirectWrite 字体支持

**要求:**
- Visual Studio 2019+ 或 MinGW

**限制:**
- 编译配置较复杂
- 某些库需要 MSVC

---

## 性能影响

### 正面影响

1. **启动速度**: 无 JVM 启动开销，快 10-100 倍
2. **内存使用**: 无 GC 开销，通常节省 30-50%
3. **解析速度**: html5ever 和 lightningcss 通常快 50-100%
4. **线程安全**: 编译时保证，无运行时检查开销

### 潜在负面影响

1. **字体渲染**: 可能略慢于 Java AWT（取决于后端）
2. **首次编译**: 比 Java 编译慢（但运行时更快）
3. **图形操作**: 取决于后端选择

---

## 迁移建议

### 从 Java 版本迁移

1. **理解所有权系统**: Rust 的核心概念
2. **重新思考设计**: 避免过度使用继承
3. **选择合适的图形后端**: skia-safe vs cairo-rs
4. **处理错误**: 使用 Result 而不是异常
5. **学习生命周期**: 管理引用的有效性

### 推荐学习资源

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

---

## 总结

Rust 版本的 CSSBox 实现了 Java 版本的核心功能，但由于语言和生态系统的差异，存在一些限制：

**主要限制:**
1. ⚠️⚠️⚠️ Java AWT 无法完全替代（最大影响）
2. ⚠️⚠️ 字体处理可能有差异
3. ⚠️⚠️ 图像渲染需要额外配置

**优势:**
1. ✅ 更好的内存安全性
2. ✅ 更快的启动和运行速度
3. ✅ 编译时线程安全保证
4. ✅ 更现代的依赖库

所有限制都在代码中用 **⚠️** 标记，并在文档中详细说明。
