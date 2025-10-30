//! CSS 标准化和默认样式表
//!
//! 对应 Java 版本的 CSSNorm 类

/// CSS 标准化工具
pub struct CSSNorm;

impl CSSNorm {
    /// 获取标准用户代理样式表
    ///
    /// 返回 CSS 2.1 规范推荐的默认样式
    pub fn std_stylesheet() -> String {
        // 基于 CSS 2.1 规范的默认样式表
        r#"
        /* HTML 5 默认样式 */
        html, body, div, span, applet, object, iframe,
        h1, h2, h3, h4, h5, h6, p, blockquote, pre,
        a, abbr, acronym, address, big, cite, code,
        del, dfn, em, img, ins, kbd, q, s, samp,
        small, strike, strong, sub, sup, tt, var,
        b, u, i, center,
        dl, dt, dd, ol, ul, li,
        fieldset, form, label, legend,
        table, caption, tbody, tfoot, thead, tr, th, td,
        article, aside, canvas, details, embed,
        figure, figcaption, footer, header, hgroup,
        menu, nav, output, ruby, section, summary,
        time, mark, audio, video {
            margin: 0;
            padding: 0;
            border: 0;
        }
        
        html, body {
            display: block;
        }
        
        h1 { font-size: 2em; margin: 0.67em 0; font-weight: bold; }
        h2 { font-size: 1.5em; margin: 0.75em 0; font-weight: bold; }
        h3 { font-size: 1.17em; margin: 0.83em 0; font-weight: bold; }
        h4 { font-size: 1em; margin: 1.12em 0; font-weight: bold; }
        h5 { font-size: 0.83em; margin: 1.5em 0; font-weight: bold; }
        h6 { font-size: 0.75em; margin: 1.67em 0; font-weight: bold; }
        
        p { display: block; margin: 1em 0; }
        
        a { color: blue; text-decoration: underline; }
        
        strong, b { font-weight: bold; }
        em, i { font-style: italic; }
        
        ul, ol { display: block; margin: 1em 0; padding-left: 40px; }
        li { display: list-item; }
        ul { list-style-type: disc; }
        ol { list-style-type: decimal; }
        
        table { display: table; border-collapse: separate; border-spacing: 2px; }
        thead { display: table-header-group; }
        tbody { display: table-row-group; }
        tfoot { display: table-footer-group; }
        tr { display: table-row; }
        td, th { display: table-cell; padding: 1px; }
        th { font-weight: bold; text-align: center; }
        "#.to_string()
    }
    
    /// 获取用户样式表
    ///
    /// 包含一些额外的 CSSBox 特定定义
    pub fn user_stylesheet() -> String {
        r#"
        /* CSSBox 特定样式 */
        * {
            box-sizing: content-box;
        }
        "#.to_string()
    }
    
    /// 获取表单元素样式表
    ///
    /// 提供表单输入字段的基本样式
    pub fn forms_stylesheet() -> String {
        r#"
        input, textarea, select, button {
            display: inline-block;
            padding: 2px;
            border: 1px solid #999;
            background: white;
        }
        
        input[type="submit"], input[type="button"], button {
            background: #f0f0f0;
            cursor: pointer;
        }
        
        input[type="checkbox"], input[type="radio"] {
            width: 13px;
            height: 13px;
        }
        "#.to_string()
    }
}
