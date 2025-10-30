# CSSBox 项目分析文档

这个目录包含了对 CSSBox 项目的全面分析文档，帮助理解项目架构、类依赖关系，以及如何用其他编程语言实现类似功能。

## 📚 文档列表

### 1. [架构分析文档](ARCHITECTURE_ANALYSIS_ZH.md) ⭐
**最详细的综合分析文档**

包含内容：
- **项目概述**: CSSBox 的功能特性和技术栈
- **架构设计**: 分层架构、包结构详解
- **核心类详解**: 91个Java类的详细说明
  - IO/DOM层 (6个类)
  - CSS处理层 (10个类)  
  - 布局层 (39个类)
  - 渲染层 (10个类)
  - AWT实现层 (9个类)
  - 应用层 (6个类)
  - 工具层 (2个类)
- **类依赖关系图**: 核心依赖流程和包间依赖
- **设计模式分析**: 工厂、策略、访问者、观察者等
- **关键算法实现**: 
  - CSS 盒模型
  - 布局算法（正常流、浮动、定位、表格）
  - 文本布局
  - 背景渲染
  - 层叠上下文
- **数据流分析**: 从输入到输出的完整流程
- **扩展点分析**: 如何自定义各个组件
- **其他语言实现指南**: 
  - Python 完整实现示例
  - JavaScript/TypeScript 实现
  - C++ 实现要点
  - Go 实现要点
  - 核心算法伪代码

### 2. [类依赖关系文档](CLASS_DEPENDENCIES_ZH.md)
**所有Java类的详细信息**

包含内容：
- 91个Java类的完整清单
- 每个类的详细信息：
  - 类型（类、抽象类、接口、枚举）
  - 文件路径
  - 继承关系
  - 实现的接口
  - 内部依赖（依赖的其他CSSBox类）
  - 外部依赖（第三方库）
  - 主要公共方法列表
- 按包分组组织，便于查阅

### 3. [实现指南](IMPLEMENTATION_GUIDE_ZH.md)
**用其他语言实现的实践指南**

包含内容：
- **核心概念**: 渲染引擎工作流程
- **架构设计**: 模块划分和接口设计
- **分语言实现建议**:
  - Python: 适合快速原型和学习
  - JavaScript/TypeScript: 适合Web应用
  - C++: 适合高性能应用
  - Go: 适合服务端渲染
- **关键算法实现**:
  - CSS选择器匹配
  - 样式层叠
  - 盒模型布局
  - 浮动和定位
- **测试策略**: 单元测试、视觉测试、性能测试
- **性能优化**: 缓存、增量更新、并行处理

## 🎯 快速导航

### 想了解 CSSBox 的整体架构？
👉 阅读 [架构分析文档](ARCHITECTURE_ANALYSIS_ZH.md)

### 想查看具体某个类的信息？
👉 查看 [类依赖关系文档](CLASS_DEPENDENCIES_ZH.md)

### 想用其他语言实现类似功能？
👉 阅读 [实现指南](IMPLEMENTATION_GUIDE_ZH.md)

## 📊 项目统计

- **总类数**: 91个
- **总包数**: 8个
- **代码行数**: 约30,000行
- **主要依赖**: 
  - jstyleparser (CSS解析)
  - neko-htmlunit (HTML解析)
  - slf4j (日志)

## 🏗️ 架构概览

```
┌─────────────────────────────────────────┐
│         应用层 (Demo/Testing)            │
│         6个类 - 示例和测试工具            │
├─────────────────────────────────────────┤
│         渲染层 (Render/AWT)              │
│         19个类 - 图形渲染实现             │
├─────────────────────────────────────────┤
│         布局层 (Layout)                  │
│         39个类 - 盒模型和布局引擎         │
├─────────────────────────────────────────┤
│         CSS处理层 (CSS)                  │
│         10个类 - 样式解析和计算           │
├─────────────────────────────────────────┤
│         IO/DOM层 (IO)                    │
│         6个类 - 文档加载和DOM处理         │
├─────────────────────────────────────────┤
│         工具层 (Misc)                    │
│         2个类 - 辅助工具                 │
└─────────────────────────────────────────┘
```

## 🔑 核心类关系

```
DocumentSource → DOMSource → DOMAnalyzer → BoxFactory → Engine → Viewport
                                ↓
                           CSS规则应用
                                ↓
                           Box树构建
                                ↓
                           ┌─────────────┐
                           │    Box      │
                           │  (抽象基类)  │
                           └──────┬──────┘
                                  │
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
              ElementBox      TextBox       LineBox
                    │
         ┌──────────┼──────────┐
         ▼          ▼          ▼
    BlockBox   InlineBox   TableBox
```

## 🌟 关键设计模式

1. **工厂模式**: `BoxFactory` 创建不同类型的盒子
2. **策略模式**: `ImageLoader` 不同的图像加载策略
3. **访问者模式**: `StructuredRenderer` 遍历盒子树
4. **观察者模式**: `ContentObserver` 监听内容变化
5. **模板方法**: `Engine` 定义布局算法框架
6. **适配器模式**: `GraphicsVisualContext` 适配AWT

## 📖 使用示例

### 基本渲染示例
```java
// 1. 创建文档源
DocumentSource docSource = new DefaultDocumentSource(url);

// 2. 解析HTML为DOM
DOMSource domSource = new DefaultDOMSource(docSource);
Document doc = domSource.parse();

// 3. 创建渲染引擎
GraphicsEngine engine = new GraphicsEngine(
    domSource.getDOM(),
    domSource,
    1200,  // 宽度
    800    // 高度
);

// 4. 执行渲染
Viewport viewport = engine.getViewport();
viewport.draw();

// 5. 获取结果
BufferedImage image = engine.getImage();
ImageIO.write(image, "png", new File("output.png"));
```

## 🔄 数据流

```
HTML文档 → 解析 → DOM树 → 应用CSS → 样式化DOM
                                      ↓
                                  创建盒子树
                                      ↓
                                  计算布局
                                      ↓
                                  渲染输出
                                      ↓
                              图像/显示
```

## 🛠️ 如何扩展

### 自定义盒子类型
```java
public class CustomBox extends BlockBox {
    @Override
    public void layout(int availw, boolean force) {
        // 自定义布局逻辑
        super.layout(availw, force);
    }
}
```

### 自定义渲染器
```java
public class CustomRenderer extends GraphicsRenderer {
    @Override
    protected void renderTextContent(TextBox box) {
        // 自定义文本渲染
        super.renderTextContent(box);
    }
}
```

## 📝 实现检查清单

如果你想用其他语言实现类似功能，建议按以下顺序：

- [ ] **阶段1**: HTML解析 + DOM树
- [ ] **阶段2**: CSS解析 + 选择器匹配
- [ ] **阶段3**: 样式计算 + 层叠
- [ ] **阶段4**: 基本盒模型（Block + Inline）
- [ ] **阶段5**: 文本布局 + 换行
- [ ] **阶段6**: 浮动布局
- [ ] **阶段7**: 定位（相对、绝对、固定）
- [ ] **阶段8**: 表格布局
- [ ] **阶段9**: 背景渲染（颜色、图片）
- [ ] **阶段10**: 边框和阴影
- [ ] **阶段11**: 渐变和高级背景
- [ ] **阶段12**: 性能优化

## 🎓 学习路径

### 初学者
1. 阅读**架构分析文档**的"项目概述"和"架构设计"部分
2. 理解基本的数据流
3. 查看简单的使用示例

### 中级开发者
1. 深入阅读**核心类详解**
2. 理解**关键算法实现**
3. 研究**设计模式**的应用

### 高级开发者
1. 研究**类依赖关系**
2. 分析**扩展点**
3. 参考**其他语言实现指南**进行移植

## 📚 相关资源

- **CSSBox 官网**: http://cssbox.sourceforge.net/
- **GitHub 仓库**: https://github.com/radkovo/CSSBox
- **jStyleParser**: http://cssbox.sourceforge.net/jstyleparser
- **W3C CSS 规范**: https://www.w3.org/Style/CSS/
- **MDN Web 文档**: https://developer.mozilla.org/

## 🤝 贡献

如果发现文档有错误或需要改进的地方，欢迎提交 Issue 或 Pull Request。

## 📄 许可

CSSBox 项目采用 GNU Lesser General Public License (LGPL) v3 许可。

---

**文档版本**: 1.0  
**最后更新**: 2025-10-30  
**CSSBox 版本**: 5.0.3-SNAPSHOT  
**文档作者**: GitHub Copilot Agent
