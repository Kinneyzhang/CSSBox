//! CSSBox Rust 使用示例
//!
//! 演示如何使用 CSSBox Rust 渲染 HTML 文档

use cssbox_rust::{
    io::DocumentSource,
    css::{DOMAnalyzer, CSSNorm, Origin},
    layout::Engine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();
    
    println!("CSSBox Rust - HTML 渲染示例");
    println!("============================\n");
    
    // 示例 1: 从字符串加载 HTML
    example_from_string()?;
    
    // 示例 2: 从 URL 加载（需要网络支持）
    // example_from_url()?;
    
    // 示例 3: 从文件加载
    // example_from_file()?;
    
    Ok(())
}

/// 示例 1: 从字符串加载 HTML
fn example_from_string() -> Result<(), Box<dyn std::error::Error>> {
    println!("示例 1: 从字符串加载 HTML\n");
    
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>CSSBox Rust 测试</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    margin: 20px;
                }
                h1 {
                    color: #333;
                    border-bottom: 2px solid #0066cc;
                }
                .content {
                    background: #f5f5f5;
                    padding: 15px;
                    border-radius: 5px;
                }
            </style>
        </head>
        <body>
            <h1>欢迎使用 CSSBox Rust</h1>
            <div class="content">
                <p>这是一个使用 Rust 重新实现的 HTML/CSS 渲染引擎。</p>
                <p>支持的功能：</p>
                <ul>
                    <li>HTML5 解析</li>
                    <li>CSS3 样式</li>
                    <li>盒模型布局</li>
                    <li>图像渲染</li>
                </ul>
            </div>
        </body>
        </html>
    "#;
    
    // 创建文档源
    let doc_source = DocumentSource::from_string(
        html.to_string(),
        "http://localhost/"
    );
    
    println!("✓ 文档源创建成功");
    println!("  基础 URL: {}", doc_source.get_url());
    
    // 创建 DOM 分析器
    let mut analyzer = DOMAnalyzer::new(doc_source.get_url().to_string());
    
    // 添加标准样式表
    analyzer.add_stylesheet(Origin::Agent, CSSNorm::std_stylesheet());
    analyzer.add_stylesheet(Origin::Agent, CSSNorm::user_stylesheet());
    
    println!("✓ DOM 分析器创建成功");
    
    // 加载样式表
    analyzer.get_stylesheets()?;
    println!("✓ 样式表加载完成");
    
    // 创建渲染引擎
    let mut engine = Engine::new(
        analyzer.get_root(),
        analyzer,
        doc_source.get_url().to_string()
    );
    
    println!("✓ 渲染引擎创建成功");
    
    // 创建布局
    let viewport_width = 1200.0;
    let viewport_height = 800.0;
    engine.create_layout(viewport_width, viewport_height)?;
    
    println!("✓ 布局计算完成");
    println!("  视口尺寸: {}x{}", viewport_width, viewport_height);
    
    // 渲染图像（需要图形库支持）
    println!("\n⚠️  图像渲染功能需要启用图形后端 (skia-safe 或 cairo)");
    println!("   使用 --features graphics 编译以启用此功能");
    
    match engine.render_to_image() {
        Ok(_) => println!("✓ 图像渲染成功"),
        Err(e) => println!("⚠️  {}", e),
    }
    
    println!("\n示例 1 完成！\n");
    Ok(())
}

/// 示例 2: 从 URL 加载 HTML
#[allow(dead_code)]
fn example_from_url() -> Result<(), Box<dyn std::error::Error>> {
    println!("示例 2: 从 URL 加载 HTML\n");
    
    let url = "http://example.com";
    
    let mut doc_source = DocumentSource::from_url(url)?;
    println!("✓ 文档源创建成功: {}", url);
    
    // 读取文档内容
    match doc_source.read() {
        Ok(content) => {
            println!("✓ 文档加载成功 ({}字节)", content.len());
        }
        Err(e) => {
            println!("⚠️  文档加载失败: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n示例 2 完成！\n");
    Ok(())
}

/// 示例 3: 从文件加载 HTML
#[allow(dead_code)]
fn example_from_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("示例 3: 从文件加载 HTML\n");
    
    let file_path = "file:///path/to/file.html";
    
    let mut doc_source = DocumentSource::from_url(file_path)?;
    println!("✓ 文档源创建成功: {}", file_path);
    
    match doc_source.read() {
        Ok(content) => {
            println!("✓ 文件读取成功 ({}字节)", content.len());
        }
        Err(e) => {
            println!("⚠️  文件读取失败: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n示例 3 完成！\n");
    Ok(())
}
