# CSSBox 架构分析文档

## 项目概述

CSSBox 是一个用纯 Java 编写的 (X)HTML/CSS 渲染引擎。它的主要目的是提供关于渲染页面的完整信息，适合进一步处理。同时，它也允许显示渲染后的文档。

### 核心特性

- **纯 Java 实现**：无需本地依赖，跨平台兼容
- **CSS 解析和应用**：完整支持 CSS 样式解析和应用
- **DOM 树构建**：从 HTML/XML 构建 DOM 树
- **布局引擎**：实现 CSS 盒模型和布局算法
- **渲染引擎**：支持图形渲染输出
- **可扩展性**：模块化设计，易于扩展

### 技术栈

- **Java 11+**：项目使用 Java 11 作为基础版本
- **Maven**：构建和依赖管理工具
- **主要依赖**：
  - `jstyleparser` (4.0.1)：CSS 解析器
  - `neko-htmlunit` (4.7.0)：HTML 解析器
  - `slf4j-api` (1.7.30)：日志框架

## 架构设计

### 整体架构

CSSBox 采用分层架构设计，主要分为以下几层：

```
┌─────────────────────────────────────────┐
│         应用层 (Demo/Testing)            │
├─────────────────────────────────────────┤
│         渲染层 (Render/AWT)              │
├─────────────────────────────────────────┤
│         布局层 (Layout)                  │
├─────────────────────────────────────────┤
│         CSS处理层 (CSS)                  │
├─────────────────────────────────────────┤
│         IO/DOM层 (IO)                    │
├─────────────────────────────────────────┤
│         工具层 (Misc)                    │
└─────────────────────────────────────────┘
```

### 包结构分析

项目包含以下主要包：

1. **org.fit.cssbox.layout** (39个类)
   - 核心布局引擎
   - 盒模型实现
   - 定位和渲染算法

2. **org.fit.cssbox.css** (10个类)
   - CSS 解析和处理
   - 样式计算
   - DOM 分析

3. **org.fit.cssbox.awt** (9个类)
   - AWT/Swing 渲染实现
   - 图形上下文
   - 图像加载

4. **org.fit.cssbox.render** (10个类)
   - 渲染抽象层
   - 背景渲染
   - 渐变支持

5. **org.fit.cssbox.io** (6个类)
   - 文档加载
   - DOM 源管理
   - 内容观察

6. **org.fit.cssbox.demo** (6个类)
   - 示例应用
   - 使用演示

7. **org.fit.cssbox.testing** (4个类)
   - 测试工具
   - 参考测试

8. **org.fit.cssbox.misc** (2个类)
   - 工具类

## 核心类详解

### 第一层：IO/DOM层

#### 1. DocumentSource (接口)
- **作用**：文档源的抽象接口
- **实现类**：
  - `DefaultDocumentSource`：默认实现，从 URL 加载
  - `StreamDocumentSource`：从流加载

#### 2. DOMSource (接口)
- **作用**：DOM 树源的抽象
- **实现类**：
  - `DefaultDOMSource`：使用 NekoHTML 解析

#### 3. ContentObserver (接口)
- **作用**：监听内容变化

### 第二层：CSS处理层

#### 4. DOMAnalyzer (类)
- **作用**：分析 DOM 树并应用 CSS 样式
- **关键功能**：
  - 样式计算
  - 继承处理
  - 伪元素处理
- **依赖**：
  - jstyleparser (CSS 解析)
  - DOM API

#### 5. CSSDecoder (类)
- **作用**：解码 CSS 属性值
- **功能**：
  - 颜色解码
  - 长度单位转换
  - 边框样式解码

#### 6. BackgroundDecoder (类)
- **作用**：解析背景相关的 CSS 属性
- **支持**：
  - 背景图片
  - 背景渐变
  - 背景定位

#### 7. FontDecoder (类)
- **作用**：字体属性解析
- **管理**：
  - 字体族
  - 字体大小
  - 字体样式

#### 8. CSSNorm (类)
- **作用**：CSS 值标准化

#### 9. HTMLNorm (类)
- **作用**：HTML 属性到 CSS 的转换

#### 10. CSSUnits (类)
- **作用**：CSS 单位转换工具

### 第三层：布局层

#### 11. Box (抽象类)
- **作用**：所有盒子的基类
- **核心概念**：CSS 盒模型的抽象
- **子类层次**：
```
Box (抽象)
├── ElementBox (抽象) - 元素盒子
│   ├── BlockBox - 块级盒子
│   │   ├── BlockReplacedBox - 块级替换盒子
│   │   ├── ListItemBox - 列表项盒子
│   │   └── BlockTableBox - 块级表格
│   ├── InlineBox - 内联盒子
│   │   ├── InlineReplacedBox - 内联替换盒子
│   │   └── InlineBlockBox - 内联块盒子
│   │       └── InlineBlockReplacedBox
│   ├── TableBox - 表格盒子
│   ├── TableBodyBox - 表格主体
│   ├── TableRowBox - 表格行
│   ├── TableCellBox - 表格单元格
│   ├── TableCaptionBox - 表格标题
│   └── Viewport - 视口
├── TextBox - 文本盒子
├── LineBox - 行盒子
└── ReplacedBox (抽象) - 替换元素盒子
```

#### 12. ElementBox (抽象类)
- **作用**：代表 DOM 元素的盒子
- **关键属性**：
  - 内容区域
  - 内边距
  - 边框
  - 外边距
  - 定位信息

#### 13. BlockBox (类)
- **作用**：块级盒子实现
- **布局特性**：
  - 垂直堆叠
  - 宽度填充父容器
  - 包含块级和内联内容

#### 14. InlineBox (类)
- **作用**：内联盒子实现
- **布局特性**：
  - 水平排列
  - 行内换行
  - 基线对齐

#### 15. TextBox (类)
- **作用**：文本内容盒子
- **功能**：
  - 文本分割
  - 空白处理
  - 字体度量

#### 16. ReplacedBox (抽象类)
- **作用**：替换元素基类（img、video等）
- **子类**：
  - `ReplacedImage`
  - `ReplacedText`
  - `BlockReplacedBox`
  - `InlineReplacedBox`

#### 17. TableBox (类)
- **作用**：表格布局实现
- **算法**：
  - 自动表格布局
  - 固定表格布局
  - 单元格合并

#### 18. Viewport (类)
- **作用**：渲染视口，根盒子
- **功能**：
  - 初始包含块
  - 视口大小管理

#### 19. Engine (抽象类)
- **作用**：布局引擎基类
- **职责**：
  - 盒子树构建
  - 布局计算
  - 渲染管理

#### 20. BoxFactory (接口)
- **作用**：盒子工厂接口
- **实现**：`HTMLBoxFactory`

#### 21. VisualContext (类)
- **作用**：视觉上下文
- **包含**：
  - 字体信息
  - 颜色
  - 文本度量

#### 22. FloatList (类)
- **作用**：管理浮动元素

#### 23. StackingContext (类)
- **作用**：层叠上下文管理
- **用途**：z-index 排序

### 第四层：渲染层

#### 24. StructuredRenderer (抽象类)
- **作用**：结构化渲染器基类
- **方法**：
  - `startElementContents()`
  - `finishElementContents()`
  - `renderElementBackground()`
  - `renderTextContent()`

#### 25. BoxRenderer (接口)
- **作用**：盒子渲染器接口

#### 26. ElementBackground (类)
- **作用**：元素背景抽象
- **支持**：
  - 多背景层
  - 背景裁剪
  - 背景原点

#### 27. BackgroundImage* (接口/类)
- `BackgroundImageImage`：图片背景
- `BackgroundImageGradient`：渐变背景

#### 28. Gradient (抽象类)
- **子类**：
  - `LinearGradient`：线性渐变
  - `RadialGradient`：径向渐变

#### 29. BackgroundRepeater (类)
- **作用**：背景重复处理

### 第五层：AWT实现层

#### 30. GraphicsEngine (类)
- **作用**：基于 AWT/Swing 的渲染引擎
- **继承**：`Engine`
- **功能**：
  - Graphics2D 渲染
  - 图像输出

#### 31. GraphicsRenderer (类)
- **作用**：具体的图形渲染实现
- **继承**：`StructuredRenderer`

#### 32. GraphicsVisualContext (类)
- **作用**：AWT 视觉上下文
- **继承**：`VisualContext`

#### 33. BrowserCanvas (类)
- **作用**：浏览器画布组件
- **继承**：`JPanel`
- **功能**：
  - 交互式显示
  - 事件处理

#### 34. GraphicsImageLoader (类)
- **作用**：图像加载器实现
- **实现**：`ImageLoader` 接口

#### 35. BackgroundBitmap (类)
- **作用**：位图背景渲染
- **继承**：`ElementBackground`

### 应用层

#### 36. Demo 类
- `BoxBrowser`：完整的浏览器演示
- `SimpleBrowser`：简单浏览器
- `ImageRenderer`：图像渲染工具
- `TextBoxes`：文本框提取
- `ComputeStyles`：样式计算演示
- `StyleImport`：样式导入工具

## 类依赖关系图

### 核心依赖流程

```
用户请求 URL
    ↓
DocumentSource (加载文档)
    ↓
DOMSource (解析 HTML → DOM)
    ↓
DOMAnalyzer (应用 CSS 样式)
    ↓
BoxFactory (创建盒子树)
    ↓
Engine (布局计算)
    ↓
Viewport → BlockBox → InlineBox → TextBox
    ↓
StructuredRenderer (渲染)
    ↓
GraphicsRenderer (绘制到 Graphics2D)
    ↓
输出 (显示/图像)
```

### 包之间的依赖关系

```
demo → awt → render → layout → css → io
                         ↓       ↓
                       misc    testing
```

## 关键设计模式

### 1. 工厂模式
- `BoxFactory` / `HTMLBoxFactory`
- 用于创建不同类型的盒子

### 2. 策略模式
- `ImageLoader`
- 不同的图像加载策略

### 3. 访问者模式
- `StructuredRenderer`
- 遍历盒子树进行渲染

### 4. 观察者模式
- `ContentObserver`
- 监听内容变化

### 5. 模板方法模式
- `Engine` 的子类实现
- 定义布局算法框架

### 6. 适配器模式
- `GraphicsVisualContext`
- 适配 AWT 的视觉上下文

## 关键算法实现

### 1. CSS 盒模型
- **文件**：`Box.java`, `ElementBox.java`
- **实现**：
  - 内容区域 (content)
  - 内边距 (padding)
  - 边框 (border)
  - 外边距 (margin)

### 2. 布局算法
- **正常流**：`BlockBox`, `InlineBox`
- **浮动**：`FloatList`
- **定位**：`ElementBox` 中的定位逻辑
- **表格**：`TableBox` 及相关类

### 3. 文本布局
- **文件**：`TextBox.java`, `LineBox.java`
- **功能**：
  - 文本分割
  - 行框创建
  - 基线对齐

### 4. 背景渲染
- **文件**：`ElementBackground.java`, `BackgroundDecoder.java`
- **支持**：
  - 多层背景
  - 背景定位
  - 背景大小
  - 渐变

### 5. 层叠上下文
- **文件**：`StackingContext.java`
- **实现**：z-index 排序和绘制顺序

## 数据流分析

### 输入阶段
1. **URL/HTML 输入** → `DocumentSource`
2. **HTML 解析** → `DOMSource` (使用 NekoHTML)
3. **DOM 树构建** → `org.w3c.dom.Document`

### 处理阶段
4. **样式应用** → `DOMAnalyzer`
   - 加载外部样式表
   - 计算最终样式
   - 处理伪元素
5. **盒子树构建** → `BoxFactory`
   - 为每个 DOM 节点创建对应的盒子
   - 建立父子关系
6. **布局计算** → `Engine`
   - 计算每个盒子的位置和大小
   - 处理浮动和定位
   - 表格布局

### 输出阶段
7. **渲染准备** → `StructuredRenderer`
   - 遍历盒子树
   - 准备渲染数据
8. **图形绘制** → `GraphicsRenderer`
   - 背景绘制
   - 边框绘制
   - 内容绘制
9. **输出** → `BufferedImage` 或 `JPanel`

## 扩展点分析

### 1. 自定义文档源
实现 `DocumentSource` 接口：
```java
public class CustomDocumentSource implements DocumentSource {
    @Override
    public InputStream getInputStream() { ... }
    @Override
    public String getContentType() { ... }
    // ...
}
```

### 2. 自定义盒子类型
继承 `ElementBox` 或其子类：
```java
public class CustomBox extends BlockBox {
    @Override
    public void layout(int availw, boolean force) { ... }
}
```

### 3. 自定义渲染器
实现 `StructuredRenderer`：
```java
public class CustomRenderer extends StructuredRenderer {
    @Override
    protected void renderElementBackground(...) { ... }
    @Override
    protected void renderTextContent(...) { ... }
}
```

### 4. 自定义图像加载
实现 `ImageLoader` 接口：
```java
public class CustomImageLoader implements ImageLoader {
    @Override
    public ContentImage loadImage(...) { ... }
}
```

## 性能考虑

### 1. 图像缓存
- `ImageCache` 接口
- `UnlimitedImageCache` 实现
- 避免重复加载

### 2. 布局优化
- 增量布局
- 避免不必要的重新计算

### 3. 渲染优化
- 层叠上下文裁剪
- 视口外元素跳过

## 如何用其他语言实现

### 整体策略

要用其他语言（如 Python、JavaScript、C++、Go）实现类似的渲染引擎，需要按以下步骤进行：

### 第一步：选择基础库

#### Python
- **HTML 解析**：`html5lib`, `lxml`, `BeautifulSoup`
- **CSS 解析**：`tinycss2`, `cssutils`
- **图形渲染**：`Pillow`, `cairo`, `skia-python`
- **GUI**：`tkinter`, `PyQt`, `wxPython`

#### JavaScript/TypeScript
- **HTML 解析**：浏览器内置 DOM API 或 `jsdom` (Node.js)
- **CSS 解析**：`css-tree`, `postcss`
- **图形渲染**：Canvas API, WebGL, `node-canvas`
- **GUI**：HTML/Canvas 直接显示

#### C++
- **HTML 解析**：`gumbo-parser`, `lexbor`
- **CSS 解析**：`katana-parser`, 自实现
- **图形渲染**：`Cairo`, `Skia`, `Qt`
- **GUI**：`Qt`, `wxWidgets`, `GTK+`

#### Go
- **HTML 解析**：`golang.org/x/net/html`
- **CSS 解析**：`github.com/andybalholm/cascadia`, 自实现
- **图形渲染**：`github.com/fogleman/gg`, `golang.org/x/image`
- **GUI**：`fyne`, `gio`

### 第二步：实现核心模块

#### 模块1：DOM 和 CSS 处理
```
功能对应：
- org.fit.cssbox.io → 文档加载模块
- org.fit.cssbox.css → CSS 解析和应用模块

关键任务：
1. 加载 HTML 文档
2. 解析为 DOM 树
3. 解析 CSS 规则
4. 应用样式到 DOM 节点
5. 计算最终样式（继承、层叠）
```

**Python 示例**：
```python
# 使用 html5lib 解析 HTML
import html5lib
from tinycss2 import parse_stylesheet

# 解析 HTML
doc = html5lib.parse(html_string)

# 解析 CSS
stylesheet = parse_stylesheet(css_string)

# 样式应用（需要实现选择器匹配）
def apply_styles(element, stylesheet):
    # 匹配选择器
    # 计算层叠
    # 计算继承
    pass
```

#### 模块2：盒模型
```
功能对应：
- org.fit.cssbox.layout.Box → 盒子基类
- org.fit.cssbox.layout.ElementBox → 元素盒子
- org.fit.cssbox.layout.BlockBox → 块盒子
- org.fit.cssbox.layout.InlineBox → 内联盒子

关键任务：
1. 定义盒子数据结构
2. 实现盒模型（content, padding, border, margin）
3. 实现不同显示类型（block, inline, inline-block）
```

**Python 示例**：
```python
from dataclasses import dataclass
from typing import Optional, List

@dataclass
class Rectangle:
    x: float
    y: float
    width: float
    height: float

@dataclass
class Box:
    element: object  # DOM 元素引用
    content: Rectangle
    padding: Rectangle
    border: Rectangle
    margin: Rectangle
    children: List['Box']
    parent: Optional['Box']
    
    display: str  # 'block', 'inline', 'inline-block', etc.
    position: str  # 'static', 'relative', 'absolute', 'fixed'
    float: str    # 'none', 'left', 'right'
```

#### 模块3：布局引擎
```
功能对应：
- org.fit.cssbox.layout.Engine → 布局引擎

关键任务：
1. 遍历 DOM 树，创建盒子树
2. 计算每个盒子的尺寸
3. 计算每个盒子的位置
4. 处理特殊布局（浮动、定位、表格）
```

**Python 示例**：
```python
class LayoutEngine:
    def __init__(self, viewport_width, viewport_height):
        self.viewport = Viewport(viewport_width, viewport_height)
    
    def create_box_tree(self, dom_root):
        """从 DOM 树创建盒子树"""
        box = self.create_box(dom_root)
        for child in dom_root.children:
            child_box = self.create_box_tree(child)
            box.children.append(child_box)
        return box
    
    def layout(self, box, available_width):
        """布局算法"""
        if box.display == 'block':
            self.layout_block(box, available_width)
        elif box.display == 'inline':
            self.layout_inline(box, available_width)
        # ...
    
    def layout_block(self, box, available_width):
        """块级布局"""
        # 1. 计算宽度
        box.content.width = self.calculate_width(box, available_width)
        
        # 2. 布局子元素
        y_offset = 0
        for child in box.children:
            self.layout(child, box.content.width)
            child.content.y = y_offset
            y_offset += child.margin.height
        
        # 3. 计算高度
        box.content.height = y_offset
```

#### 模块4：渲染引擎
```
功能对应：
- org.fit.cssbox.render → 渲染抽象层
- org.fit.cssbox.awt → AWT 渲染实现

关键任务：
1. 遍历盒子树
2. 绘制背景
3. 绘制边框
4. 绘制内容（文本、图像）
```

**Python 示例（使用 Pillow）**：
```python
from PIL import Image, ImageDraw, ImageFont

class Renderer:
    def __init__(self, width, height):
        self.image = Image.new('RGB', (width, height), 'white')
        self.draw = ImageDraw.Draw(self.image)
    
    def render_box(self, box):
        """渲染一个盒子"""
        # 1. 绘制背景
        if box.background_color:
            self.draw.rectangle(
                [box.border.x, box.border.y,
                 box.border.x + box.border.width,
                 box.border.y + box.border.height],
                fill=box.background_color
            )
        
        # 2. 绘制边框
        if box.border_width > 0:
            self.draw.rectangle(
                [box.border.x, box.border.y,
                 box.border.x + box.border.width,
                 box.border.y + box.border.height],
                outline=box.border_color,
                width=box.border_width
            )
        
        # 3. 绘制内容
        if box.is_text:
            font = ImageFont.truetype(box.font_family, box.font_size)
            self.draw.text(
                (box.content.x, box.content.y),
                box.text_content,
                fill=box.color,
                font=font
            )
        
        # 4. 递归渲染子元素
        for child in box.children:
            self.render_box(child)
    
    def save(self, filename):
        self.image.save(filename)
```

### 第三步：实现关键算法

#### 1. CSS 选择器匹配
```python
def matches_selector(element, selector):
    """检查元素是否匹配选择器"""
    # 实现选择器匹配逻辑
    # 支持：标签选择器、类选择器、ID选择器、
    #      属性选择器、伪类、组合选择器等
    pass
```

#### 2. 样式层叠
```python
def cascade_styles(element, stylesheets):
    """计算元素的最终样式"""
    styles = {}
    
    # 1. 收集所有匹配的规则
    rules = []
    for sheet in stylesheets:
        for rule in sheet.rules:
            if matches_selector(element, rule.selector):
                rules.append(rule)
    
    # 2. 按特异性和来源排序
    rules.sort(key=lambda r: r.specificity)
    
    # 3. 应用规则
    for rule in rules:
        styles.update(rule.declarations)
    
    # 4. 处理继承
    if element.parent:
        for prop in INHERITED_PROPERTIES:
            if prop not in styles:
                styles[prop] = element.parent.computed_style.get(prop)
    
    return styles
```

#### 3. 文本换行
```python
def line_break(text, available_width, font):
    """文本换行算法"""
    lines = []
    current_line = ""
    
    words = text.split()
    for word in words:
        test_line = current_line + " " + word if current_line else word
        width = measure_text(test_line, font)
        
        if width <= available_width:
            current_line = test_line
        else:
            if current_line:
                lines.append(current_line)
            current_line = word
    
    if current_line:
        lines.append(current_line)
    
    return lines
```

#### 4. 浮动布局
```python
class FloatManager:
    def __init__(self):
        self.left_floats = []
        self.right_floats = []
    
    def add_float(self, box, side):
        """添加浮动元素"""
        if side == 'left':
            self.left_floats.append(box)
        elif side == 'right':
            self.right_floats.append(box)
    
    def get_available_width(self, y):
        """获取给定 y 坐标处的可用宽度"""
        left_offset = 0
        right_offset = 0
        
        for float_box in self.left_floats:
            if self.overlaps_y(float_box, y):
                left_offset = max(left_offset, 
                                 float_box.margin.x + float_box.margin.width)
        
        for float_box in self.right_floats:
            if self.overlaps_y(float_box, y):
                right_offset = max(right_offset,
                                  self.viewport_width - float_box.margin.x)
        
        return self.viewport_width - left_offset - right_offset
```

### 第四步：集成和优化

#### 完整流程示例（Python）

```python
class Browser:
    def __init__(self, width=800, height=600):
        self.width = width
        self.height = height
    
    def load_url(self, url):
        """加载并渲染 URL"""
        # 1. 加载文档
        html = self.fetch(url)
        
        # 2. 解析 HTML
        dom = self.parse_html(html)
        
        # 3. 加载和解析 CSS
        stylesheets = self.load_stylesheets(dom)
        
        # 4. 应用样式
        self.apply_styles(dom, stylesheets)
        
        # 5. 创建盒子树
        box_tree = self.create_layout_tree(dom)
        
        # 6. 布局
        self.layout(box_tree, self.width)
        
        # 7. 渲染
        image = self.render(box_tree)
        
        return image
    
    def fetch(self, url):
        import urllib.request
        with urllib.request.urlopen(url) as response:
            return response.read().decode('utf-8')
    
    def parse_html(self, html):
        import html5lib
        return html5lib.parse(html)
    
    def load_stylesheets(self, dom):
        # 解析 <style> 标签和 <link> 标签
        stylesheets = []
        # ... 实现
        return stylesheets
    
    def apply_styles(self, dom, stylesheets):
        # 为每个元素计算最终样式
        # ... 实现
        pass
    
    def create_layout_tree(self, dom):
        engine = LayoutEngine(self.width, self.height)
        return engine.create_box_tree(dom)
    
    def layout(self, box_tree, width):
        engine = LayoutEngine(self.width, self.height)
        engine.layout(box_tree, width)
    
    def render(self, box_tree):
        renderer = Renderer(self.width, self.height)
        renderer.render_box(box_tree)
        return renderer.image

# 使用
browser = Browser()
image = browser.load_url('http://example.com')
image.save('output.png')
```

### 简化实现策略

对于学习或原型开发，可以采用以下简化策略：

1. **最小 CSS 支持**
   - 只支持基本属性：color, font-size, width, height, margin, padding
   - 忽略复杂选择器
   - 不支持伪类、伪元素

2. **简化布局**
   - 只实现 block 和 inline
   - 不支持浮动和定位
   - 不支持表格

3. **基础渲染**
   - 只支持纯色背景
   - 只支持实线边框
   - 只支持单一字体

4. **逐步扩展**
   - 第一版：基本 HTML + 简单 CSS
   - 第二版：添加浮动和定位
   - 第三版：添加表格
   - 第四版：添加复杂背景和边框
   - 第五版：添加高级选择器

### 测试策略

1. **单元测试**
   - CSS 解析器
   - 选择器匹配
   - 样式层叠
   - 布局算法

2. **视觉测试**
   - 渲染简单 HTML 页面
   - 与参考渲染比较
   - 使用图像差异工具

3. **参考测试**
   - CSS Working Group 测试套件
   - 浏览器兼容性测试

## 使用示例

### 基本用法

```java
// 1. 创建文档源
DocumentSource docSource = new DefaultDocumentSource(url);

// 2. 创建 DOM 源
DOMSource domSource = new DefaultDOMSource(docSource);
Document doc = domSource.parse();

// 3. 创建渲染引擎
BrowserConfig config = new BrowserConfig();
Dimension windowSize = new Dimension(1200, 800);
GraphicsEngine engine = new GraphicsEngine(
    domSource.getDOM(),
    domSource,
    windowSize.width,
    windowSize.height
);

// 4. 创建视口
Viewport viewport = engine.getViewport();
viewport.draw();

// 5. 渲染到图像
BufferedImage image = engine.getImage();
ImageIO.write(image, "png", new File("output.png"));
```

### 高级用法

```java
// 自定义盒子工厂
BoxFactory factory = new HTMLBoxFactory() {
    @Override
    public ElementBox createBox(Element element) {
        // 自定义逻辑
        return super.createBox(element);
    }
};

// 自定义渲染器
GraphicsRenderer renderer = new GraphicsRenderer(engine) {
    @Override
    protected void renderTextContent(TextBox box) {
        // 自定义文本渲染
        super.renderTextContent(box);
    }
};

// 使用自定义组件
engine.setBoxFactory(factory);
engine.setRenderer(renderer);
```

## 总结

CSSBox 是一个设计良好的 HTML/CSS 渲染引擎，具有以下特点：

### 优点
1. **纯 Java 实现**，跨平台兼容
2. **模块化设计**，易于理解和扩展
3. **清晰的架构**，分层明确
4. **良好的抽象**，接口和实现分离
5. **完整的 CSS 支持**，包括渐变、多背景等高级特性

### 适用场景
1. **PDF 生成**：将 HTML 转换为 PDF
2. **截图服务**：网页截图
3. **HTML 邮件渲染**
4. **测试工具**：CSS 兼容性测试
5. **学习参考**：学习渲染引擎实现

### 改进方向
1. **性能优化**：增量渲染、GPU 加速
2. **标准兼容**：更完整的 CSS3 支持
3. **JavaScript 支持**：动态内容处理
4. **更好的文档**：API 文档和示例

如果要用其他语言实现类似功能，建议：
1. 从简单的 HTML + CSS 子集开始
2. 逐步添加功能
3. 参考 CSSBox 的架构设计
4. 使用现有的解析库
5. 编写充分的测试

---

**文档版本**：1.0  
**最后更新**：2025-10-30  
**CSSBox 版本**：5.0.3-SNAPSHOT
