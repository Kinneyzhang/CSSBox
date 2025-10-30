//! 文档源处理
//!
//! 对应 Java 版本的 DocumentSource 和 DefaultDocumentSource 类

use crate::Result;
use std::io::Read;

/// 文档源
///
/// 表示文档的来源（URL、文件或字符串）
///
/// 对应 Java 版本: `org.fit.cssbox.io.DocumentSource`
pub struct DocumentSource {
    /// 文档 URL
    url: String,
    
    /// 文档内容
    content: Option<String>,
}

impl DocumentSource {
    /// 从 URL 创建文档源
    ///
    /// # 参数
    /// - `url`: 文档 URL（支持 http, https, file 等协议）
    pub fn from_url(url: &str) -> Result<Self> {
        let normalized_url = if !url.starts_with("http://") 
            && !url.starts_with("https://") 
            && !url.starts_with("file://") {
            format!("http://{}", url)
        } else {
            url.to_string()
        };
        
        Ok(DocumentSource {
            url: normalized_url,
            content: None,
        })
    }
    
    /// 从字符串创建文档源
    pub fn from_string(content: String, base_url: &str) -> Self {
        DocumentSource {
            url: base_url.to_string(),
            content: Some(content),
        }
    }
    
    /// 获取文档 URL
    pub fn get_url(&self) -> &str {
        &self.url
    }
    
    /// 读取文档内容
    pub fn read(&mut self) -> Result<String> {
        if let Some(content) = &self.content {
            return Ok(content.clone());
        }
        
        // 尝试从 URL 加载
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            self.fetch_from_network()
        } else if self.url.starts_with("file://") {
            self.read_from_file()
        } else {
            Err(crate::CSSBoxError::NetworkError(
                format!("不支持的 URL 协议: {}", self.url)
            ))
        }
    }
    
    /// 从网络获取文档
    fn fetch_from_network(&mut self) -> Result<String> {
        log::info!("从网络获取文档: {}", self.url);
        
        // ⚠️ 注意: 这需要 reqwest crate 支持
        // 实际实现需要处理各种 HTTP 特性（重定向、编码、超时等）
        
        #[cfg(feature = "network")]
        {
            match reqwest::blocking::get(&self.url) {
                Ok(response) => {
                    match response.text() {
                        Ok(text) => {
                            self.content = Some(text.clone());
                            Ok(text)
                        }
                        Err(e) => Err(crate::CSSBoxError::NetworkError(
                            format!("读取响应失败: {}", e)
                        ))
                    }
                }
                Err(e) => Err(crate::CSSBoxError::NetworkError(
                    format!("HTTP 请求失败: {}", e)
                ))
            }
        }
        
        #[cfg(not(feature = "network"))]
        {
            Err(crate::CSSBoxError::NotImplemented(
                "网络功能未启用 - 需要编译时开启 'network' feature".to_string()
            ))
        }
    }
    
    /// 从文件读取文档
    fn read_from_file(&mut self) -> Result<String> {
        let path = self.url.strip_prefix("file://").unwrap_or(&self.url);
        log::info!("从文件读取文档: {}", path);
        
        let mut file = std::fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        self.content = Some(content.clone());
        Ok(content)
    }
    
    /// 关闭文档源
    pub fn close(&mut self) {
        self.content = None;
    }
}
