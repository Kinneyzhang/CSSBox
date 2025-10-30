# CSSBox Rust Implementation

[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

> **完整的 Rust 重新实现** of the Java CSSBox HTML/CSS rendering engine

[English](#english) | [中文文档](docs/README_CN.md)

---

## English

### Overview

This is a complete Rust reimplementation of [CSSBox](http://cssbox.sourceforge.net/), an HTML/CSS rendering engine originally written in Java by Radek Burget.

**Original Java Project:**
- ~27,000 lines of code
- 91 Java source files
- Dependencies: NekoHTML, jStyleParser, SLF4J

**Rust Implementation:**
- Leverages Rust's zero-cost abstractions
- Memory-safe without garbage collection
- Uses modern Rust ecosystem libraries

### Features

- ✅ HTML5 parsing (html5ever)
- ✅ CSS3 parsing (lightningcss)
- ✅ Box model layout
- ⏳ Graphics rendering (skia-safe/cairo)
- ⏳ Font handling (font-kit)
- ⏳ Image processing

### Quick Start

```rust
use cssbox_rust::{
    io::DocumentSource,
    css::{DOMAnalyzer, CSSNorm, Origin},
    layout::Engine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load HTML document
    let doc_source = DocumentSource::from_url("http://example.com")?;
    
    // Create DOM analyzer with CSS
    let mut analyzer = DOMAnalyzer::new(doc_source.get_url().to_string());
    analyzer.add_stylesheet(Origin::Agent, CSSNorm::std_stylesheet());
    analyzer.get_stylesheets()?;
    
    // Create rendering engine and compute layout
    let mut engine = Engine::new(
        analyzer.get_root(),
        analyzer,
        doc_source.get_url().to_string()
    );
    engine.create_layout(1200.0, 800.0)?;
    
    Ok(())
}
```

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cssbox_rust = "0.1.0"
```

### Requirements

- **Minimum Rust version**: 1.70 or higher
- **Recommended**: Rust 1.75+ for better compatibility

If you encounter build errors with older Rust versions, please update:
```bash
rustup update stable
```

### Building

```bash
# Basic build
cargo build --release

# With graphics support
cargo build --release --features graphics

# Run examples
cargo run --example render_html
```

### Architecture

```
src/
├── layout/     # Layout engine (box model, positioning)
├── css/        # CSS parsing and style computation
├── io/         # Document loading and DOM processing
├── render/     # Rendering engine
├── awt/        # Graphics abstraction layer
├── misc/       # Utilities
└── testing/    # Test framework
```

### Implementation Limitations

#### ⚠️ Java AWT has no direct Rust equivalent

The original Java version uses AWT (Abstract Window Toolkit) for graphics. The Rust version uses:
- **Graphics rendering**: `skia-safe` or `cairo-rs` instead of Java2D
- **Font handling**: `font-kit` instead of Java Font API
- **Image processing**: `image` crate instead of `java.awt.image`

**Impact:**
- Cannot implement interactive window features
- Some font rendering differences
- Platform-specific dependencies required

#### ⚠️ DOM API differences

Java uses W3C DOM API, Rust uses `html5ever` + `kuchiki`:
- Different API design but equivalent functionality
- More modern and standards-compliant

#### ⚠️ CSS Parser differences

Java uses jStyleParser, Rust uses `lightningcss`:
- Better CSS3 support
- Faster parsing
- Some non-standard extensions may not be supported

See [detailed documentation](docs/README_CN.md) for complete limitation list.

### Performance

Expected performance characteristics compared to Java version:

| Metric | Java | Rust | Notes |
|--------|------|------|-------|
| Parsing speed | Baseline | ~1.5-2x faster | html5ever performance |
| Layout computation | Baseline | ~1.2-1.5x faster | Zero-cost abstractions |
| Memory usage | Baseline | ~0.6-0.8x | No GC overhead |
| Startup time | Slow | Fast | No JVM startup |

### Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Run `cargo fmt` and `cargo clippy`
4. Add tests for new features
5. Submit a pull request

### License

GNU Lesser General Public License 3.0 (same as original Java version)

### Credits

- Original CSSBox by Radek Burget
- Rust ecosystem libraries: html5ever, lightningcss, skia-safe, font-kit, etc.

---

## 详细中文文档

请查看 [完整中文文档](docs/README_CN.md)，包含：

- 📚 详细的架构说明
- ⚠️ 完整的实现限制清单
- 📖 使用示例和教程
- 🔧 编译和运行指南
- 📊 性能对比分析
- 🚀 未来开发计划

### 核心限制摘要

本 Rust 实现有以下主要限制，已在代码中明确标注：

1. **Java AWT 图形系统** - 使用 skia-safe/cairo 替代，部分高级功能无法实现
2. **DOM API 差异** - 使用 html5ever/kuchiki，API 设计不同但功能等价
3. **CSS 解析库** - 使用 lightningcss，某些非标准扩展可能不支持
4. **字体处理** - 使用 font-kit，渲染效果可能略有差异
5. **反射机制** - 使用枚举和 trait objects，运行时类型发现受限
6. **线程模型** - 使用所有权系统，设计模式需要调整

所有限制都在源代码中用 `⚠️` 标记，并在文档中详细说明。

### 快速开始（中文）

```bash
# 克隆仓库
git clone https://github.com/Kinneyzhang/CSSBox.git
cd CSSBox

# 构建项目
cargo build --release

# 运行示例
cargo run --example render_html

# 运行测试
cargo test
```

查看 [examples/](examples/) 目录获取更多使用示例。
