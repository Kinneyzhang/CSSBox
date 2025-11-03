# CSSBox 架构分析 - 用于 Emacs Lisp 重新实现

## 概述

CSSBox 是一个用 Java 编写的 HTML/CSS 渲染引擎，它可以解析 HTML 文档、应用 CSS 样式、计算布局并渲染视觉输出。本文档提供了全面的架构分析，旨在指导使用 Emacs Lisp 进行函数式重新实现，使用包前缀 `etml`（ETML - Emacs Text Markup Language）。

**代码总量**：约 27,000 行 Java 代码，分布在 91 个源文件中
**核心复杂度**：中高
**主要依赖**：jStyleParser（CSS 解析）、NekoHTML（HTML 解析）

---

## 1. 系统架构概述

### 1.1 高层架构

CSSBox 遵循流水线架构，包含以下主要阶段：

```
Document Loading → DOM Parsing → Style Analysis → Layout Computation → Rendering
文档加载 → DOM 解析 → 样式分析 → 布局计算 → 渲染
```

**关键架构模式：**
- **Factory Pattern**（工厂模式）：BoxFactory 创建适当的盒子类型
- **Composite Pattern**（组合模式）：Box 树层次结构（Box → ElementBox → BlockBox 等）
- **Strategy Pattern**（策略模式）：不同的渲染器（GraphicsRenderer、StructuredRenderer）
- **Visitor Pattern**（访问者模式）：用于布局和渲染的树遍历
- **Template Method**（模板方法）：抽象 Engine 类及其具体实现

### 1.2 包结构和职责

```
org.fit.cssbox/
├── io/          (文档加载和解析 - 453 LOC)
├── css/         (样式分析和 CSS 处理 - 2411 LOC)
├── layout/      (Box 模型和布局引擎 - 17195 LOC) ★ 核心
├── render/      (渲染后端 - 1905 LOC)
├── awt/         (Java AWT 特定渲染 - 2073 LOC)
├── demo/        (示例应用程序 - 1798 LOC)
├── misc/        (工具类 - 188 LOC)
└── testing/     (测试框架 - 882 LOC)
```

**★ `layout` 包是系统的核心** - 占代码库的 63%。

---

## 2. 核心组件深度解析

### 2.1 Document Loading Pipeline（文档加载流水线 - io 包）

**目的**：从 URL 或流加载和解析 HTML 文档。

**关键类：**

1. **DocumentSource**（抽象类）
   - 职责：HTML 内容的抽象源
   - 关键方法：`getURL()`、`getInputStream()`、`close()`
   - 实现：
     - `DefaultDocumentSource`：HTTP/HTTPS/File URL 加载
     - `StreamDocumentSource`：直接流输入

2. **DOMSource**（抽象类）
   - 职责：将 HTML 解析为 DOM 树
   - 关键方法：`parse()` → 返回 `org.w3c.dom.Document`
   - 实现：
     - `DefaultDOMSource`：使用 NekoHTML parser

**函数式 Elisp 映射：**
```elisp
;; 纯函数 - 无全局状态
(defun etml-document-source-create (url-or-path)
  "Create document source descriptor."
  (list :type (if (string-match-p "^https?://" url-or-path) 'url 'file)
        :location url-or-path
        :encoding 'utf-8))

(defun etml-document-source-fetch (source)
  "Fetch content from SOURCE, returns string."
  (pcase (plist-get source :type)
    ('url (etml--fetch-url (plist-get source :location)))
    ('file (etml--read-file (plist-get source :location)))))

(defun etml-dom-parse (html-string)
  "Parse HTML string into DOM tree (libxml-parse-html-region)."
  (with-temp-buffer
    (insert html-string)
    (libxml-parse-html-region (point-min) (point-max))))
```

**实现注意事项：**
- Emacs 有内置的 `libxml-parse-html-region` 用于 HTML 解析
- 使用 `url-retrieve-synchronously` 进行 HTTP 获取
- 返回不可变数据结构（plists/alists）表示 DOM 节点

---

### 2.2 Style Analysis（样式分析 - css 包）

**目的**：加载 CSS、计算元素样式、处理层叠和继承。

**关键类：**

1. **DOMAnalyzer**（2411 LOC - 最复杂的类）
   - 职责：中央 CSS 处理协调器
   - 关键操作：
     - 加载和解析 CSS 样式表
     - 将 HTML 属性转换为 CSS（例如 `<font>` 标签）
     - 计算每个元素的有效样式
     - 处理 CSS cascade（用户代理 → 用户 → 作者）
     - 管理 media queries
   
   **关键方法：**
   ```java
   - attributesToStyles()           // HTML→CSS 转换
   - addStyleSheet(url, css, origin) // 添加样式表
   - getStyleSheets()               // 加载链接的样式表
   - getElementStyle(element)       // 获取计算样式
   ```

2. **CSSNorm**
   - 标准样式表的静态方法
   - `stdStyleSheet()`：HTML 默认样式
   - `userStyleSheet()`：额外默认样式
   - `formsStyleSheet()`：表单元素样式

3. **FontDecoder** / **FontTable**
   - 字体解析和管理
   - 处理 @font-face 规则

4. **CSSUnits**
   - 单位转换（px、em、rem、%、pt 等）
   - 上下文相关计算

**函数式 Elisp 映射：**
```elisp
(defun etml-css-normalize-html-attributes (dom-node)
  "Convert HTML presentation attributes to CSS properties."
  (let ((tag (dom-tag dom-node))
        (attrs (dom-attributes dom-node)))
    ;; 返回添加了 :style 属性的新节点
    (etml--merge-inline-styles 
     dom-node
     (etml--html-attrs-to-css tag attrs))))

(defun etml-css-parse-stylesheet (css-string base-url)
  "Parse CSS string into rule list."
  ;; 使用内置 css-mode 解析或外部库
  (etml--css-tokenize-and-parse css-string))

(defun etml-css-compute-style (dom-node stylesheets inherited-styles)
  "Compute effective style for DOM-NODE.
Returns plist of CSS properties."
  (let* ((matched-rules (etml--match-selectors dom-node stylesheets))
         (cascaded (etml--apply-cascade matched-rules))
         (inherited (etml--inherit-properties inherited-styles))
         (inline (etml--extract-inline-style dom-node)))
    (etml--merge-styles inherited cascaded inline)))

(defun etml-css-unit-to-px (value unit context)
  "Convert CSS value with UNIT to pixels.
CONTEXT contains font-size, dpi, parent-width for relative units."
  (pcase unit
    ('px value)
    ('em (* value (plist-get context :font-size)))
    ('rem (* value (plist-get context :root-font-size)))
    ('% (* value (plist-get context :reference-value) 0.01))
    (_ (error "Unknown unit: %s" unit))))
```

**实现策略：**
- **CSS 解析**：考虑使用外部 CSS 解析器或实现基本解析器
- **选择器匹配**：实现 CSS 选择器引擎（使用 `dom.el` 辅助函数）
- **Cascade 算法**：三个有序列表（user-agent、user、author），按特异性合并
- **继承**：定义可继承属性，向下传播树

**复杂度警告**：这是最复杂的子系统。从最小 CSS 支持开始：
1. 基本选择器（tag、class、id、descendant）
2. 核心属性（display、position、width、height、margin、padding）
3. 颜色和字体
4. 逐步添加高级功能

---

### 2.3 Layout Engine（布局引擎 - layout 包）- 核心部分

**目的**：将样式化的 DOM 转换为具有计算尺寸的定位盒树。

这是**代码库的 63%**，也是最关键的组件。

#### 2.3.1 Box Model Hierarchy（盒模型层次结构）

**类层次结构：**
```
Box (abstract)
├── TextBox (leaf)
└── ElementBox (abstract)
    ├── InlineBox
    │   ├── InlineReplacedBox (<img>, <input>, etc.)
    │   └── InlineBlockBox
    ├── BlockBox
    │   ├── BlockReplacedBox
    │   ├── BlockTableBox
    │   ├── ListItemBox
    │   └── Viewport (root)
    └── TableBox, TableRowBox, TableCellBox, etc.
```

**Box 的关键属性：**
```java
- Node node;              // DOM 节点
- Rectangle bounds;       // 相对于父级的位置
- Rectangle absbounds;    // 绝对位置
- ElementBox parent;      // 父盒子
- ElementBox cbox;        // 包含块
- VisualContext ctx;      // 字体、颜色等
- boolean isblock;        // Block vs inline
- boolean displayed;      // display !== none
- boolean visible;        // visibility !== hidden
```

**函数式 Elisp 映射：**
```elisp
(cl-defstruct etml-box
  "Base box structure."
  node           ; DOM 节点引用
  bounds         ; (x y width height) 相对于父级
  abs-bounds     ; (x y width height) 绝对位置
  parent         ; 父 etml-box
  containing-block ; 定位用的包含块
  context        ; etml-visual-context
  is-block       ; t 表示 block，nil 表示 inline
  displayed      ; t 如果 display !== none
  visible        ; t 如果 visibility !== hidden
  children)      ; 子盒子列表

(cl-defstruct (etml-element-box (:include etml-box))
  "Element box with style."
  element        ; DOM 元素
  style          ; 计算的 CSS 属性 (plist)
  margin         ; (top right bottom left)
  border         ; (top right bottom left)
  padding        ; (top right bottom left))

(cl-defstruct (etml-block-box (:include etml-element-box))
  "Block-level box."
  content-width
  content-height
  floats)        ; 浮动盒子列表

(cl-defstruct (etml-inline-box (:include etml-element-box))
  "Inline box."
  line-boxes)    ; 行盒子列表

(cl-defstruct (etml-text-box (:include etml-box))
  "Text content box."
  text           ; 字符串内容
  font-metrics)  ; 字体大小、行高等

;; 纯函数创建盒子
(defun etml-create-box (dom-node style parent-box)
  "Create appropriate box type based on computed STYLE."
  (let ((display (plist-get style :display)))
    (cond
     ((eq dom-node 'text-node)
      (make-etml-text-box
       :node dom-node
       :text (etml--extract-text dom-node)
       :parent parent-box))
     
     ((eq display 'block)
      (make-etml-block-box
       :node dom-node
       :style style
       :parent parent-box
       :is-block t))
     
     ((eq display 'inline)
      (make-etml-inline-box
       :node dom-node
       :style style
       :parent parent-box
       :is-block nil))
     
     ;; ... 处理其他 display 类型
     )))
```


#### 2.3.2 BoxFactory - Box 树构建

**目的**：将 DOM 树转换为 Box 树。

**关键算法：**
```java
1. 遍历 DOM 树（深度优先）
2. 对于每个元素：
   a. 获取计算样式
   b. 根据 display 属性创建适当的 Box 子类
   c. 处理匿名盒子（包装 inline→block 转换）
   d. 处理伪元素（::before、::after）
   e. 递归处理子元素
3. 处理特殊情况：
   - 表格：插入匿名表格包装器
   - 列表：创建列表标记
   - 替换元素：加载内容
```

**函数式 Elisp 映射：**
```elisp
(defun etml-build-box-tree (dom-tree stylesheets viewport-size)
  "Build box tree from DOM-TREE with applied STYLESHEETS.
Returns root etml-viewport box."
  (let* ((root-element (etml--find-root-element dom-tree))
         (viewport (etml--create-viewport viewport-size))
         (style-map (etml--compute-all-styles dom-tree stylesheets)))
    ;; 递归构建树
    (etml--build-box-subtree root-element viewport style-map)
    viewport))

(defun etml--build-box-subtree (dom-node parent-box style-map)
  "Recursively build box subtree for DOM-NODE."
  (let* ((style (gethash dom-node style-map))
         (display (plist-get style :display)))
    (cond
     ;; 跳过 display:none
     ((eq display 'none) nil)
     
     ;; 创建盒子并处理子元素
     (t
      (let ((box (etml-create-box dom-node style parent-box)))
        ;; 处理子元素
        (dolist (child (dom-children dom-node))
          (when-let ((child-box (etml--build-box-subtree 
                                  child box style-map)))
            (push child-box (etml-box-children box))))
        ;; 返回带子元素的盒子
        box)))))
```

#### 2.3.3 Layout Computation - 宽度/高度计算

**这是最复杂的算法。** 它实现了 CSS 2.1 盒模型和定位。

**关键阶段：**

1. **宽度计算**（自顶向下）
   ```java
   - 可用宽度从父级流向子级
   - 块宽度：相对于父级解析 %
   - 内联宽度：子元素之和 + padding/border/margin
   ```

2. **高度计算**（自底向上）
   ```java
   - 子元素首先布局
   - 父高度 = 子元素高度之和
   - 除非设置了显式高度
   ```

3. **位置计算**
   ```java
   - 正常流：顺序定位
   - 浮动：复杂的浮动盒子算法
   - 绝对定位：相对于包含块
   - 固定定位：相对于视口
   ```

**函数式 Elisp 映射：**
```elisp
(defun etml-layout-box (box available-width)
  "Compute layout for BOX with AVAILABLE-WIDTH.
Returns box with computed bounds. Pure function."
  (cond
   ((etml-block-box-p box)
    (etml--layout-block-box box available-width))
   
   ((etml-inline-box-p box)
    (etml--layout-inline-box box available-width))
   
   ((etml-text-box-p box)
    (etml--layout-text-box box available-width))))

(defun etml--layout-block-box (box available-width)
  "Layout block box. Returns new box with computed dimensions."
  (let* ((style (etml-block-box-style box))
         ;; 宽度计算
         (width (etml--compute-width style available-width))
         ;; 布局子元素
         (laid-out-children 
          (etml--layout-children 
           (etml-box-children box) 
           width))
         ;; 高度是子元素之和（如果没有显式设置）
         (height (or (etml--explicit-height style)
                     (etml--sum-heights laid-out-children)))
         ;; 计算位置
         (x (etml--compute-x-position style box))
         (y (etml--compute-y-position style box laid-out-children)))
    ;; 返回带布局的新盒子
    (etml--copy-box-with-bounds
     box
     :bounds (list x y width height)
     :children laid-out-children)))

(defun etml--compute-width (style available-width)
  "Compute width from style and available width."
  (let ((width-prop (plist-get style :width))
        (margin-left (plist-get style :margin-left))
        (margin-right (plist-get style :margin-right))
        (padding (plist-get style :padding))
        (border (plist-get style :border-width)))
    (cond
     ;; 显式宽度
     ((numberp width-prop) width-prop)
     
     ;; 百分比
     ((and (consp width-prop) (eq (car width-prop) 'percent))
      (* available-width (/ (cdr width-prop) 100.0)))
     
     ;; Auto - 填充可用空间
     ((eq width-prop 'auto)
      (- available-width 
         (etml--sum-horizontal-spacing margin-left margin-right 
                                       padding border))))))

(defun etml--layout-inline-box (box available-width)
  "Layout inline box - complex line breaking algorithm."
  ;; 这非常复杂 - 涉及：
  ;; 1. 将内联内容收集到行盒子中
  ;; 2. 在 available-width 处断行
  ;; 3. 行内的垂直对齐
  ;; 4. 处理浮动
  (etml--build-line-boxes box available-width))
```

**关键实现注意事项：**

1. **不可变性**：所有布局函数应返回新盒子，而不是修改现有盒子
2. **两遍算法**： 
   - 第一遍：宽度（自顶向下）
   - 第二遍：高度和位置（自底向上）
3. **浮动很难**：浮动布局是最复杂的 CSS 功能之一
4. **断行**：对于内联内容，实现正确的断行算法

#### 2.3.4 Viewport - 根容器

**目的**：顶级容器，管理可见区域与总画布。

```elisp
(cl-defstruct (etml-viewport (:include etml-block-box))
  "Root viewport box."
  canvas-width    ; 总页面宽度
  canvas-height   ; 总页面高度
  visible-rect    ; (x y width height) 可见区域
  config)         ; 浏览器配置

(defun etml-create-viewport (width height)
  "Create root viewport."
  (make-etml-viewport
   :bounds (list 0 0 width height)
   :abs-bounds (list 0 0 width height)
   :canvas-width width
   :canvas-height height
   :visible-rect (list 0 0 width height)
   :is-block t
   :children nil))
```

#### 2.3.5 Visual Context（视觉上下文）

**目的**：在树中流动的渲染状态（字体、颜色等）。

```elisp
(cl-defstruct etml-visual-context
  "Visual rendering context."
  font-size       ; 当前字体大小（px）
  font-family     ; 字体系列名称
  font-weight     ; normal、bold 等
  color           ; 文本颜色
  background      ; 背景颜色
  line-height     ; 行高倍数
  dpi             ; 屏幕 DPI，用于单位转换
  viewport)       ; viewport 引用

(defun etml-create-context (parent-context style)
  "Create new context inheriting from PARENT-CONTEXT with STYLE."
  (make-etml-visual-context
   :font-size (or (plist-get style :font-size)
                  (etml-visual-context-font-size parent-context))
   :font-family (or (plist-get style :font-family)
                    (etml-visual-context-font-family parent-context))
   ;; ... 其他属性
   :viewport (etml-visual-context-viewport parent-context)))
```

---

### 2.4 Rendering（渲染 - render 包）

**目的**：将布局好的盒树转换为视觉输出。

**关键类：**

1. **BoxRenderer**（接口）
   - `startElement(box)` / `endElement(box)`
   - `renderTextContent(box)`
   
2. **StructuredRenderer**（抽象类）
   - 树遍历逻辑
   - 渲染的访问者模式

3. **GraphicsRenderer**（AWT 实现）
   - 绘制到 Java Graphics2D
   - 处理裁剪、变换

**函数式 Elisp 映射：**
```elisp
(defun etml-render-to-buffer (box-tree buffer)
  "Render BOX-TREE to BUFFER with text properties."
  (with-current-buffer buffer
    (erase-buffer)
    (etml--render-box box-tree)))

(defun etml--render-box (box)
  "Recursively render BOX."
  (cond
   ((etml-text-box-p box)
    (etml--render-text box))
   
   ((etml-element-box-p box)
    (etml--render-element-start box)
    (dolist (child (etml-box-children box))
      (etml--render-box child))
    (etml--render-element-end box))))

(defun etml--render-text (text-box)
  "Insert text with text properties."
  (let* ((text (etml-text-box-text text-box))
         (context (etml-box-context text-box))
         (face (etml--context-to-face context))
         (start (point)))
    (insert text)
    (put-text-property start (point) 'face face)))

(defun etml--context-to-face (context)
  "Convert visual context to Emacs face."
  (list :family (etml-visual-context-font-family context)
        :height (* 10 (etml-visual-context-font-size context))
        :weight (etml-visual-context-font-weight context)
        :foreground (etml-visual-context-color context)
        :background (etml-visual-context-background context)))
```

**替代渲染目标：**

1. **带文本属性的 Emacs Buffer**
   - 简单，使用内置文本属性
   - 有限的视觉保真度（无精确定位）

2. **SVG 渲染**
   - Emacs 可以显示内联 SVG
   - 精确定位
   ```elisp
   (defun etml-render-to-svg (box-tree width height)
     "Render BOX-TREE to SVG string."
     (concat
      (format "<svg width="%d" height="%d">\n" width height)
      (etml--render-boxes-to-svg (etml-viewport-children box-tree))
      "</svg>"))
   
   (defun etml--render-box-to-svg (box)
     "Render single box to SVG element."
     (let ((bounds (etml-box-bounds box)))
       (format "<rect x="%f" y="%f" width="%f" height="%f" fill="%s"/>\n"
               (nth 0 bounds) (nth 1 bounds)
               (nth 2 bounds) (nth 3 bounds)
               (or (plist-get (etml-element-box-style box) :background-color)
                   "transparent"))))
   ```

3. **使用 ImageMagick 的图像渲染**
   - 如果 Emacs 编译时支持 ImageMagick
   - 可以创建实际的位图图像

---

## 3. 关键算法详解

### 3.1 CSS Cascade 算法

**目的**：当多个规则匹配时确定哪个 CSS 规则获胜。

**算法：**
```
1. 收集所有匹配元素的规则
2. 对于每个规则，计算特异性：
   - 内联样式：特异性 = (1,0,0,0)
   - ID：(0,1,0,0)
   - Class/attribute/pseudo-class：(0,0,1,0)
   - Element/pseudo-element：(0,0,0,1)
3. 排序依据：
   a. 来源（user-agent < user < author < inline）
   b. !important 标志
   c. 特异性
   d. 源顺序
4. 按排序顺序应用（后面的覆盖前面的）
```

**Elisp 实现：**
```elisp
(defun etml-css-compute-specificity (selector)
  "Compute CSS specificity for SELECTOR.
Returns (id-count class-count element-count)."
  (let ((id-count 0)
        (class-count 0)
        (element-count 0))
    (dolist (component (etml--parse-selector selector))
      (pcase (car component)
        ('id (cl-incf id-count))
        ((or 'class 'attribute 'pseudo-class) (cl-incf class-count))
        ('element (cl-incf element-count))))
    (list id-count class-count element-count)))

(defun etml-css-compare-specificity (spec1 spec2)
  "Compare two specificity tuples.
Returns positive if spec1 > spec2, negative if spec1 < spec2, 0 if equal."
  (or (- (nth 0 spec1) (nth 0 spec2))  ; ID count
      (- (nth 1 spec1) (nth 1 spec2))  ; Class count
      (- (nth 2 spec1) (nth 2 spec2)))) ; Element count

(defun etml-css-apply-cascade (rules)
  "Apply cascade algorithm to RULES.
Returns merged property list."
  (let ((sorted-rules 
         (sort rules
               (lambda (r1 r2)
                 (let ((spec1 (etml-css-rule-specificity r1))
                       (spec2 (etml-css-rule-specificity r2)))
                   (< (etml-css-compare-specificity spec1 spec2) 0))))))
    ;; 按顺序合并属性
    (cl-reduce #'etml--merge-properties
               sorted-rules
               :key #'etml-css-rule-properties
               :initial-value nil)))
```

### 3.2 Box Layout 算法（简化版）

**目的**：定位盒子并计算尺寸。

**主算法：**
```
function layout(box, availableWidth):
  1. 计算宽度：
     - 如果有显式宽度：使用它
     - 如果是百分比：从 availableWidth 计算
     - 如果是 auto（block）：填充可用空间
     - 如果是 auto（inline）：收缩到内容
  
  2. 布局子元素：
     - 对于块子元素：
       * 垂直堆叠
       * 每个获得完整的内容宽度
     - 对于内联子元素：
       * 水平打包成行
       * 在 availableWidth 处断行
  
  3. 计算高度：
     - 如果有显式高度：使用它
     - 否则：子元素高度之和
  
  4. 定位盒子：
     - 正常流：在前一个兄弟之后
     - 浮动：复杂的浮动定位
     - 绝对定位：相对于包含块
  
  5. 返回设置了边界的盒子
```

**Elisp 实现（简化版）：**
```elisp
(defun etml-layout-box-tree (root-box viewport-width)
  "Layout entire box tree starting from ROOT-BOX."
  ;; 两遍算法
  (let* ((pass1-box (etml--layout-pass-width root-box viewport-width))
         (pass2-box (etml--layout-pass-height pass1-box)))
    pass2-box))

(defun etml--layout-pass-width (box available-width)
  "First pass: compute widths top-down."
  (let* ((style (when (etml-element-box-p box) 
                  (etml-element-box-style box)))
         (width (if (etml-block-box-p box)
                    (etml--compute-block-width style available-width)
                  available-width)))
    ;; 在盒子上设置宽度
    (setf (nth 2 (etml-box-bounds box)) width)
    ;; 使用新宽度布局子元素
    (dolist (child (etml-box-children box))
      (etml--layout-pass-width child width))
    box))

(defun etml--layout-pass-height (box)
  "Second pass: compute heights bottom-up."
  ;; 首先布局所有子元素
  (dolist (child (etml-box-children box))
    (etml--layout-pass-height child))
  ;; 然后计算此盒子的高度
  (let ((height (etml--compute-height box)))
    (setf (nth 3 (etml-box-bounds box)) height)
    ;; 定位子元素
    (etml--position-children box)
    box))

(defun etml--compute-height (box)
  "Compute height of box based on children."
  (cond
   ;; 显式高度
   ((when-let ((style (and (etml-element-box-p box)
                           (etml-element-box-style box))))
      (plist-get style :height)))
   
   ;; 文本盒子：使用字体度量
   ((etml-text-box-p box)
    (etml--text-height box))
   
   ;; 子元素之和
   (t
    (cl-reduce #'+ (etml-box-children box)
               :key (lambda (child) (nth 3 (etml-box-bounds child)))
               :initial-value 0))))

(defun etml--position-children (parent-box)
  "Set x,y positions for children of PARENT-BOX."
  (let ((y 0))  ; 运行 Y 位置
    (dolist (child (etml-box-children parent-box))
      (let ((bounds (etml-box-bounds child)))
        ;; 设置 Y 位置
        (setf (nth 1 bounds) y)
        ;; 推进 Y
        (cl-incf y (nth 3 bounds))))))  ; height
```

### 3.3 选择器匹配算法

**目的**：确定 CSS 选择器是否匹配元素。

```elisp
(defun etml-css-selector-matches-p (selector element dom-tree)
  "Return t if SELECTOR matches ELEMENT in DOM-TREE."
  (let ((parts (etml--parse-selector selector)))
    (etml--match-selector-parts parts element dom-tree)))

(defun etml--match-selector-parts (parts element dom-tree)
  "Match compound selector PARTS against ELEMENT."
  (cond
   ;; 空选择器 - 匹配
   ((null parts) t)
   
   ;; 后代组合器：'div p'
   ((eq (car parts) 'descendant)
    (and (etml--match-simple-selector (cadr parts) element)
         (etml--has-ancestor-matching (cddr parts) element dom-tree)))
   
   ;; 子组合器：'div > p'
   ((eq (car parts) 'child)
    (and (etml--match-simple-selector (cadr parts) element)
         (when-let ((parent (etml--get-parent element dom-tree)))
           (etml--match-selector-parts (cddr parts) parent dom-tree))))
   
   ;; 简单选择器：'div'、'.class'、'#id'
   (t (etml--match-simple-selector parts element))))

(defun etml--match-simple-selector (selector element)
  "Match simple SELECTOR against ELEMENT."
  (pcase (car selector)
    ('element (etml--match-tag (cadr selector) element))
    ('class (etml--has-class (cadr selector) element))
    ('id (etml--has-id (cadr selector) element))
    ('attribute (etml--match-attribute (cadr selector) element))))

(defun etml--match-tag (tag element)
  "Check if ELEMENT has TAG."
  (eq (dom-tag element) (intern tag)))

(defun etml--has-class (class element)
  "Check if ELEMENT has CLASS."
  (when-let ((class-attr (dom-attr element 'class)))
    (member class (split-string class-attr))))

(defun etml--has-id (id element)
  "Check if ELEMENT has ID."
  (equal id (dom-attr element 'id)))
```

---

## 4. 函数式编程适配

### 4.1 消除全局状态

**Java 问题：**
- 可变盒树
- 共享缓存（ImageCache、FontTable）
- 全局配置（BrowserConfig）

**Elisp 解决方案：**

1. **不可变数据结构**
   ```elisp
   ;; 不要修改盒子，而是返回新盒子
   (defun etml-set-box-bounds (box new-bounds)
     "Return new box with NEW-BOUNDS."
     (let ((new-box (copy-etml-box box)))
       (setf (etml-box-bounds new-box) new-bounds)
       new-box))
   ```

2. **通过函数线程化状态**
   ```elisp
   ;; 不使用全局配置，而是作为参数传递
   (defun etml-render-document (url config)
     "Render document at URL with CONFIG."
     (let* ((source (etml-document-source-create url))
            (html (etml-document-source-fetch source))
            (dom (etml-dom-parse html))
            (stylesheets (etml-css-load-stylesheets dom url config))
            (box-tree (etml-build-box-tree 
                       dom stylesheets 
                       (plist-get config :viewport-size)))
            (laid-out (etml-layout-box-tree 
                       box-tree 
                       (car (plist-get config :viewport-size)))))
       laid-out))
   ```

3. **缓存作为数据，而非副作用**
   ```elisp
   ;; 返回缓存作为结果的一部分
   (defun etml-load-image (url cache)
     "Load image from URL using CACHE.
   Returns (image . new-cache)."
     (or (gethash url cache)
         (let ((image (etml--fetch-image url)))
           (cons image (puthash url image (copy-hash-table cache))))))
   
   ;; 使用多值
   (cl-multiple-value-bind (image new-cache)
       (etml-load-image "http://..." image-cache)
     ;; 使用 image，向前传递 new-cache
     ...)
   ```

### 4.2 纯函数设计模式

**模式 1：转换，不要修改**
```elisp
;; 不好（可变）
(defun bad-layout-box (box)
  (setf (etml-box-width box) 100)
  box)

;; 好（不可变）
(defun etml-layout-box (box)
  (etml--copy-box box :width 100))
```

**模式 2：显式上下文传递**
```elisp
;; 显式传递所有依赖
(defun etml-layout-element (element style context parent-box)
  "Layout ELEMENT with STYLE in CONTEXT, child of PARENT-BOX."
  ;; 所有输入都是参数，没有全局变量
  ...)
```

**模式 3：嵌套上下文**
```elisp
;; 上下文向下流动树
(defun etml-render-tree (box context)
  (let ((child-context (etml-create-child-context context box)))
    (dolist (child (etml-box-children box))
      (etml-render-tree child child-context))))
```

---

## 5. 实现路线图

### 阶段 1：基础（2-3 周）

**目标**：基本的 HTML 解析和 DOM 处理。

```elisp
;; 交付成果：
(etml-parse-html STRING)              ; → DOM 树
(etml-dom-query DOM SELECTOR)         ; → 匹配节点
(etml-dom-traverse DOM FN)            ; 树遍历
```

**要创建的文件：**
- `etml-dom.el` - DOM 工具
- `etml-html-parser.el` - HTML 解析包装器

**复杂度**：低
**代码行数**：~500

### 阶段 2：CSS 基础（3-4 周）

**目标**：解析和匹配基本 CSS。

```elisp
;; 交付成果：
(etml-css-parse STYLESHEET)           ; → 规则列表
(etml-css-match-rules RULES ELEMENT)  ; → 匹配规则
(etml-css-compute-style ELEMENT RULES); → 样式 plist
```

**要创建的文件：**
- `etml-css-parser.el` - CSS 解析
- `etml-css-selector.el` - 选择器匹配
- `etml-css-cascade.el` - Cascade 算法
- `etml-css-properties.el` - 属性定义

**复杂度**：中高
**代码行数**：~2000

**首先实现的子集：**
- 选择器：tag、class、id、descendant、child
- 属性：display、width、height、margin、padding、border
- 单位：px、%、em
- 颜色：命名颜色、十六进制

### 阶段 3：盒模型（4-5 周）

**目标**：创建盒树并计算布局。

```elisp
;; 交付成果：
(etml-build-box-tree DOM STYLES)      ; → 盒树
(etml-layout-boxes BOX-TREE SIZE)     ; → 布局完成的树
```

**要创建的文件：**
- `etml-box.el` - Box 结构
- `etml-box-factory.el` - Box 创建
- `etml-layout-block.el` - 块布局
- `etml-layout-inline.el` - 内联布局（简化）
- `etml-layout-position.el` - 定位

**复杂度**：高
**代码行数**：~3000

**v1 的简化：**
- 无浮动
- 无绝对/固定定位
- 无表格
- 简化的内联布局（无断行）

### 阶段 4：渲染（2-3 周）

**目标**：显示布局完成的盒子。

```elisp
;; 交付成果：
(etml-render-to-buffer BOX-TREE)      ; → 带文本属性的 buffer
(etml-render-to-svg BOX-TREE)         ; → SVG 字符串
```

**要创建的文件：**
- `etml-render-buffer.el` - Buffer 渲染
- `etml-render-svg.el` - SVG 渲染

**复杂度**：中等
**代码行数**：~800

### 阶段 5：高级功能（持续）

**逐步添加：**
1. 浮动
2. 绝对/固定定位  
3. 表格
4. 正确的断行
5. 更多 CSS 属性（字体、背景、边框）
6. 图像
7. 表单
8. JavaScript 钩子（可能）

---

## 6. Elisp 函数命名约定

### 6.1 公共 API（前缀：`etml-`）

**文档处理：**
```elisp
(etml-parse-html html-string)
(etml-parse-url url)
(etml-render-document url &optional config)
```

**DOM 操作：**
```elisp
(etml-dom-query dom selector)
(etml-dom-traverse dom function)
(etml-dom-get-attribute node attr)
(etml-dom-get-text node)
```

**CSS 操作：**
```elisp
(etml-css-parse stylesheet-string)
(etml-css-compute-style element rules)
(etml-css-match-selector selector element)
```

**盒模型：**
```elisp
(etml-create-box-tree dom styles viewport-size)
(etml-layout-box-tree box-tree)
(etml-box-bounds box)
(etml-box-children box)
```

**渲染：**
```elisp
(etml-render-to-buffer box-tree buffer)
(etml-render-to-svg box-tree)
(etml-render-to-string box-tree)
```

### 6.2 内部函数（前缀：`etml--`）

**对内部使用双横线：**
```elisp
(etml--parse-selector selector)
(etml--match-simple-selector selector element)
(etml--compute-width style available-width)
(etml--layout-block-box box width)
(etml--position-children parent-box)
```

### 6.3 类型构造器（前缀：`make-etml-`）

**由 cl-defstruct 生成：**
```elisp
(make-etml-box ...)
(make-etml-block-box ...)
(make-etml-visual-context ...)
```

### 6.4 类型谓词（后缀：`-p`）

**由 cl-defstruct 生成：**
```elisp
(etml-box-p obj)
(etml-block-box-p obj)
(etml-element-box-p obj)
```

### 6.5 常量（前缀：`etml-`，无后缀）

```elisp
(defconst etml-default-font-size 16)
(defconst etml-default-line-height 1.2)
(defconst etml-named-colors '((red . "#FF0000") ...))
```

### 6.6 配置（前缀：`etml-`，无后缀）

```elisp
(defcustom etml-viewport-width 1200
  "Default viewport width in pixels."
  :type 'integer
  :group 'etml)

(defcustom etml-viewport-height 800
  "Default viewport height in pixels."
  :type 'integer
  :group 'etml)
```

---

## 7. 模块结构

**推荐的文件组织：**

```
etml/
├── etml.el                  ; 主入口点，autoloads
├── etml-core.el             ; 核心类型和工具
├── etml-dom.el              ; DOM 工具
├── etml-html-parser.el      ; HTML 解析
├── etml-css-parser.el       ; CSS 解析
├── etml-css-selector.el     ; 选择器匹配
├── etml-css-cascade.el      ; Cascade 算法  
├── etml-css-properties.el   ; 属性定义
├── etml-css-units.el        ; 单位转换
├── etml-box.el              ; Box 结构
├── etml-box-factory.el      ; Box 树创建
├── etml-layout.el           ; 布局入口点
├── etml-layout-block.el     ; 块布局
├── etml-layout-inline.el    ; 内联布局
├── etml-layout-position.el  ; 定位
├── etml-render.el           ; 渲染入口点
├── etml-render-buffer.el    ; Buffer 渲染
├── etml-render-svg.el       ; SVG 渲染
└── etml-test.el             ; 测试
```

**主入口点（`etml.el`）：**
```elisp
;;; etml.el --- HTML/CSS rendering engine for Emacs -*- lexical-binding: t -*-

;; Copyright (C) 2025 Your Name

;; Author: Your Name <your.email@example.com>
;; Version: 0.1.0
;; Package-Requires: ((emacs "27.1") (cl-lib "0.5"))
;; Keywords: html, css, rendering
;; URL: https://github.com/yourusername/etml

;;; Commentary:

;; ETML 是一个纯 Emacs Lisp 实现的 HTML/CSS 渲染引擎，受 CSSBox 库启发。
;; 它提供函数来解析 HTML、应用 CSS 样式、计算布局并渲染到各种目标。

;;; Code:

(require 'etml-core)
(require 'etml-html-parser)
(require 'etml-css)
(require 'etml-box)
(require 'etml-layout)
(require 'etml-render)

;;;###autoload
(defun etml-render-url (url)
  "Fetch and render HTML document at URL."
  (interactive "sURL: ")
  (let* ((html (etml--fetch-url url))
         (dom (etml-parse-html html))
         (config (etml--default-config))
         (box-tree (etml-render-document-internal dom url config))
         (buffer (get-buffer-create "*ETML Render*")))
    (etml-render-to-buffer box-tree buffer)
    (switch-to-buffer buffer)))

(provide 'etml)
;;; etml.el ends here
```

---

## 8. 测试策略

### 8.1 单元测试

**独立测试每个模块：**

```elisp
;;; etml-test.el

(require 'ert)
(require 'etml)

;; CSS Parser 测试
(ert-deftest etml-test-css-parse-simple ()
  "Test parsing simple CSS rule."
  (let ((css "div { color: red; }")
        (expected '((:selector "div" 
                     :properties (:color "#FF0000")))))
    (should (equal (etml-css-parse css) expected))))

;; 选择器匹配测试
(ert-deftest etml-test-selector-match-tag ()
  "Test tag selector matching."
  (let ((dom (etml-parse-html "<div><p>Test</p></div>"))
        (p-element (car (etml-dom-query dom "p"))))
    (should (etml-css-match-selector "p" p-element dom))
    (should-not (etml-css-match-selector "div" p-element dom))))

;; 布局测试
(ert-deftest etml-test-block-width ()
  "Test block width computation."
  (let* ((box (make-etml-block-box 
               :style '(:width 100 :margin-left 10 :margin-right 10)))
         (laid-out (etml--compute-block-width box 200)))
    (should (= (nth 2 (etml-box-bounds laid-out)) 100))))

;; 集成测试
(ert-deftest etml-test-render-simple-document ()
  "Test rendering simple HTML document."
  (let* ((html "<html><body><p>Hello, world!</p></body></html>")
         (dom (etml-parse-html html))
         (box-tree (etml-create-box-tree dom nil '(800 600))))
    (should (etml-viewport-p box-tree))
    (should (> (length (etml-box-children box-tree)) 0))))
```

### 8.2 视觉回归测试

**比较渲染输出：**

```elisp
(defun etml-test-visual (html-file expected-output-file)
  "Test visual rendering of HTML-FILE against EXPECTED-OUTPUT-FILE."
  (let* ((html (etml--read-file html-file))
         (dom (etml-parse-html html))
         (box-tree (etml-create-box-tree dom nil '(800 600)))
         (svg (etml-render-to-svg box-tree))
         (expected (etml--read-file expected-output-file)))
    (should (equal svg expected))))
```

---

## 9. 关键挑战和解决方案

### 挑战 1：CSS 解析复杂性

**问题**：CSS 有复杂的语法（media queries、pseudo-classes 等）

**解决方案：**
1. **使用现有库**：考虑从 JS 移植简单的 CSS 解析器
2. **子集方法**：从基本 CSS2.1 开始，逐步添加功能
3. **基于正则表达式的解析器**：对于初始版本，正则表达式可以处理简单规则

### 挑战 2：布局算法复杂性

**问题**：CSS 布局（特别是浮动、定位）非常复杂

**解决方案：**
1. **从简单开始**：仅正常流（无浮动，无绝对定位）
2. **两遍算法**：宽度自顶向下，高度自底向上
3. **参考实现**：使用 CSSBox Java 代码作为参考
4. **测试用例**：使用 W3C CSS 测试套件进行验证

### 挑战 3：Emacs 中的字体度量

**问题**：需要准确的字体度量来进行文本布局

**解决方案：**
```elisp
(defun etml--measure-text (text face)
  "Measure width and height of TEXT in FACE."
  (with-temp-buffer
    (insert (propertize text 'face face))
    (let* ((width (car (window-text-pixel-size 
                        nil (point-min) (point-max))))
           (height (cdr (window-text-pixel-size 
                         nil (point-min) (point-max)))))
      (list width height))))
```

### 挑战 4：不可变性性能

**问题**：复制盒树的开销很大

**解决方案：**
1. **结构共享**：仅对修改的部分使用 `copy-sequence`
2. **惰性求值**：按需计算布局
3. **记忆化**：缓存布局结果
4. **可接受的权衡**：v1 中正确性 > 性能

---

## 10. 依赖和库

### 10.1 内置 Emacs 库

**已经可用：**
- `dom.el` - DOM 操作
- `libxml` - HTML/XML 解析
- `url` - HTTP 获取
- `svg` - SVG 生成

### 10.2 外部依赖（可选）

**考虑使用：**
- `request.el` - 更好的 HTTP 库
- `s.el` - 字符串操作工具
- `dash.el` - 列表操作工具
- `ht.el` - 哈希表工具

### 10.3 依赖管理

```elisp
;; 在 etml.el 头部：
;; Package-Requires: ((emacs "27.1") (cl-lib "0.5"))
```

**最小依赖方法**：核心功能仅使用内置 Emacs 库。

---

## 11. 性能考虑

### 11.1 预期性能

**基准（简单文档）：**
- 解析：< 100ms
- 样式计算：< 200ms
- 布局：< 300ms
- 渲染：< 500ms
- **总计：典型页面约 1 秒**

### 11.2 优化策略

1. **惰性求值**
   ```elisp
   (defun etml-box-get-bounds (box)
     "Get bounds, computing lazily if needed."
     (or (etml-box--cached-bounds box)
         (setf (etml-box--cached-bounds box)
               (etml--compute-bounds box))))
   ```

2. **记忆化**
   ```elisp
   (defun etml-css-compute-style-memoized (element rules)
     "Compute style with memoization."
     (or (gethash element etml--style-cache)
         (puthash element
                  (etml-css-compute-style element rules)
                  etml--style-cache)))
   ```

3. **增量更新**（未来）
   - 仅重新布局更改的子树
   - 脏标志传播

### 11.3 内存使用

**典型页面的估计内存：**
- DOM 树：~50 KB
- 样式映射：~100 KB
- 盒树：~200 KB
- **总计：每页约 350 KB**

---

## 12. 示例使用模式

### 模式 1：将 URL 渲染到 Buffer

```elisp
(etml-render-url "https://example.com")
;; 在新 buffer 中打开渲染的内容
```

### 模式 2：渲染 HTML 字符串

```elisp
(let* ((html "<html><body><h1>Title</h1><p>Content</p></body></html>")
       (dom (etml-parse-html html))
       (config (list :viewport-size '(800 600)
                     :font-size 16))
       (box-tree (etml-render-document-internal dom nil config)))
  (etml-render-to-buffer box-tree (current-buffer)))
```

### 模式 3：自定义渲染

```elisp
(defun my-custom-renderer (url)
  "Custom rendering with SVG output."
  (let* ((html (etml--fetch-url url))
         (dom (etml-parse-html html))
         (box-tree (etml-create-box-tree dom nil '(1200 800)))
         (laid-out (etml-layout-box-tree box-tree 1200))
         (svg (etml-render-to-svg laid-out)))
    ;; 将 SVG 插入 buffer
    (insert-image (create-image svg 'svg t))))
```

### 模式 4：提取结构化数据

```elisp
(defun etml-extract-links (url)
  "Extract all links from page at URL."
  (let* ((html (etml--fetch-url url))
         (dom (etml-parse-html html))
         (links (etml-dom-query dom "a[href]")))
    (mapcar (lambda (link) (dom-attr link 'href)) links)))
```

---

## 13. 与 CSSBox 的比较

### 相似之处
- 相同的整体架构（parse → style → layout → render）
- 类似的盒模型层次结构
- 两遍布局算法
- 盒子创建的工厂模式

### 差异

| 方面 | CSSBox (Java) | ETML (Elisp) |
|------|---------------|--------------|
| **语言** | Java (OOP) | Elisp (函数式) |
| **可变性** | 可变盒子 | 不可变（或最小可变） |
| **状态** | 全局缓存 | 通过函数线程化 |
| **继承** | 类层次结构 | 结构组合 |
| **渲染** | Java2D Graphics | Emacs 文本属性 / SVG |
| **依赖** | jStyleParser, NekoHTML | 内置 libxml, dom.el |
| **目标** | 通用目的 | Emacs 特定 |
| **性能** | 快速（复杂页面约 100ms） | 较慢（约 1s 可接受） |
| **完整性** | 完整 CSS 2.1 + CSS3 子集 | 最初仅最小 CSS 子集 |

### 范围缩减

**v1 不需要的 CSSBox 功能：**
- 完整 CSS3 支持（渐变、变换等）
- JavaScript 执行
- 表单交互
- SVG 内联渲染
- PDF 输出
- 详细的字体度量（使用 Emacs face 系统）
- 网络缓存（Emacs 处理）
- 线程（Emacs 是单线程的）

---

## 14. 参考资料

### 文档
- **CSSBox Manual**：`/doc/manual/manual.html`
- **CSSBox API**：生成的 JavaDoc
- **CSS 2.1 规范**：https://www.w3.org/TR/CSS21/
- **W3C CSS 测试套件**：https://test.csswg.org/

### 需要研究的关键 CSSBox 类
1. `DOMAnalyzer.java` - CSS 处理
2. `BoxFactory.java` - Box 创建
3. `ElementBox.java` - 基础元素盒子
4. `BlockBox.java` - 块布局
5. `Viewport.java` - 根容器
6. `Engine.java` - 主协调器

### Emacs 资源
- **Emacs Lisp 手册**：`(info "(elisp)")` 
- **dom.el**：内置 DOM 工具
- **文本属性**：`(info "(elisp) Text Properties")`
- **SVG**：`(info "(elisp) SVG Images")`

### 测试资源
- **W3C CSS 测试套件**：全面的测试用例
- **Acid 测试**：Acid1、Acid2 用于浏览器兼容性
- **测试 HTML**：`/doc/examples/` 中的简单测试用例

---

## 15. 快速开始开发指南

### 步骤 1：设置项目

```bash
mkdir etml
cd etml
git init
touch etml.el etml-core.el etml-dom.el
```

### 步骤 2：定义核心类型

```elisp
;; etml-core.el
(cl-defstruct etml-box ...)
(cl-defstruct (etml-element-box (:include etml-box)) ...)
```

### 步骤 3：实现 HTML 解析器

```elisp
;; etml-html-parser.el
(defun etml-parse-html (html-string)
  (with-temp-buffer
    (insert html-string)
    (libxml-parse-html-region (point-min) (point-max))))
```

### 步骤 4：基本 CSS（从这里开始）

```elisp
;; etml-css-parser.el
(defun etml-css-parse (css-string)
  ;; 解析 "selector { prop: value; }" 格式
  ...)
```

### 步骤 5：简单布局

```elisp
;; etml-layout.el
(defun etml-layout-box-tree (box-tree width)
  ;; 实现基本块堆叠
  ...)
```

### 步骤 6：Buffer 渲染

```elisp
;; etml-render-buffer.el
(defun etml-render-to-buffer (box-tree buffer)
  ;; 插入带 faces 的文本
  ...)
```

### 步骤 7：测试！

```elisp
(ert-run-tests-interactively "etml-")
```

---

## 16. 结论

### 要点总结

1. **CSSBox 是一个架构良好的 HTML/CSS 渲染引擎**，具有清晰的关注点分离
2. **核心是布局引擎**（占代码 63%）- 将实现工作集中在这里
3. **CSS cascade 和选择器匹配**复杂但定义明确
4. **函数式适配**通过不可变数据和显式状态传递是可行的
5. **从小处着手**：基本 HTML + 最小 CSS → 逐步扩展
6. **使用 Emacs 内置功能**：libxml、dom.el、文本属性、SVG

### 估计总工作量

**对于最小可行实现：**
- **核心引擎**：8-10 周（CSS + 布局 + 渲染）
- **高级功能**：持续（浮动、表格、表单等）
- **总行数**：v1 约 6,000-8,000 LOC Elisp

### v1 成功标准

- [ ] 解析简单 HTML 文档
- [ ] 应用基本 CSS（tag、class、id 选择器）
- [ ] 计算块元素布局（无浮动）
- [ ] 渲染到带文本属性的 Emacs buffer
- [ ] 处理基本属性（width、height、margin、padding、color、font-size）
- [ ] 函数式 API（无全局状态）
- [ ] 50+ 测试用例的测试套件

### 后续步骤

1. **从 `etml-core.el` 开始**：定义盒子结构
2. **构建 `etml-html-parser.el`**：包装 libxml
3. **实现 `etml-css-selector.el`**：选择器匹配（最重要）
4. **开发 `etml-layout-block.el`**：简单块堆叠
5. **创建 `etml-render-buffer.el`**：文本属性渲染
6. **持续测试**：为每个功能添加测试

### 最终建议

**这是一个重要的项目（v1 需要 2-3 个月）。** CSSBox 代码库成熟且结构良好，是一个优秀的参考。函数式适配到 Elisp 是直接的，但需要严格的不可变性。从最小的可能子集开始，逐步扩展。最初优先考虑正确性而非性能。

祝重新实现顺利！🚀

---

## 附录 A：函数名称索引

**完整的推荐公共函数列表：**

### 文档处理
```elisp
etml-parse-html
etml-parse-url
etml-render-document
etml-render-url
```

### DOM 操作
```elisp
etml-dom-query
etml-dom-query-all
etml-dom-traverse
etml-dom-get-attr
etml-dom-set-attr
etml-dom-get-text
etml-dom-children
etml-dom-parent
```

### CSS 操作
```elisp
etml-css-parse
etml-css-parse-inline
etml-css-compute-style
etml-css-compute-specificity
etml-css-match-selector
etml-css-load-stylesheet
etml-css-unit-to-px
```

### 盒模型
```elisp
etml-create-box-tree
etml-box-bounds
etml-box-abs-bounds
etml-box-children
etml-box-parent
etml-box-style
etml-box-is-block-p
```

### 布局
```elisp
etml-layout-box-tree
etml-layout-box
etml-compute-dimensions
etml-compute-position
```

### 渲染
```elisp
etml-render-to-buffer
etml-render-to-svg
etml-render-to-string
etml-render-box
```

### 配置
```elisp
etml-default-config
etml-create-config
etml-config-set
etml-config-get
```

### 工具
```elisp
etml-color-parse
etml-color-to-hex
etml-font-parse
etml-unit-parse
```

---

**架构分析结束**

*文档版本：1.0*
*日期：2025-11-03*
*总页数：约 40*
*字数：约 8,500*
