# CSSBox Architecture Analysis for Emacs Lisp Reimplementation

## Executive Summary

CSSBox is an HTML/CSS rendering engine written in Java that parses HTML documents, applies CSS styles, computes layouts, and renders visual output. This document provides a comprehensive architectural analysis to guide a functional reimplementation in Emacs Lisp with the package prefix `etml` (ETML - Emacs Text Markup Language).

**Total Code Size**: ~27,000 lines of Java code across 91 source files
**Core Complexity**: Medium-High
**Primary Dependencies**: jStyleParser (CSS parsing), NekoHTML (HTML parsing)

---

## 1. System Architecture Overview

### 1.1 High-Level Architecture

CSSBox follows a pipeline architecture with these main phases:

```
Document Loading → DOM Parsing → Style Analysis → Layout Computation → Rendering
```

**Key Architectural Patterns:**
- **Factory Pattern**: BoxFactory creates appropriate box types
- **Composite Pattern**: Box tree hierarchy (Box → ElementBox → BlockBox, etc.)
- **Strategy Pattern**: Different renderers (GraphicsRenderer, StructuredRenderer)
- **Visitor Pattern**: Tree traversal for layout and rendering
- **Template Method**: Abstract Engine class with concrete implementations

### 1.2 Package Structure and Responsibilities

```
org.fit.cssbox/
├── io/          (Document loading and parsing - 453 LOC)
├── css/         (Style analysis and CSS processing - 2411 LOC)
├── layout/      (Box model and layout engine - 17195 LOC) ★ CORE
├── render/      (Rendering backends - 1905 LOC)
├── awt/         (Java AWT-specific rendering - 2073 LOC)
├── demo/        (Example applications - 1798 LOC)
├── misc/        (Utilities - 188 LOC)
└── testing/     (Testing framework - 882 LOC)
```

**★ The `layout` package is the heart of the system** - 63% of the codebase.

---

## 2. Core Components Deep Dive

### 2.1 Document Loading Pipeline (io package)

**Purpose**: Load and parse HTML documents from URLs or streams.

**Key Classes:**

1. **DocumentSource** (abstract)
   - Responsibility: Abstract source of HTML content
   - Key Methods: `getURL()`, `getInputStream()`, `close()`
   - Implementations:
     - `DefaultDocumentSource`: HTTP/HTTPS/File URL loading
     - `StreamDocumentSource`: Direct stream input

2. **DOMSource** (abstract)
   - Responsibility: Parse HTML into DOM tree
   - Key Methods: `parse()` → returns `org.w3c.dom.Document`
   - Implementation:
     - `DefaultDOMSource`: Uses NekoHTML parser

**Functional Elisp Mapping:**
```elisp
;; Pure functions - no global state
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

**Implementation Notes:**
- Emacs has built-in `libxml-parse-html-region` for HTML parsing
- Use `url-retrieve-synchronously` for HTTP fetching
- Return immutable data structures (plists/alists) for DOM nodes

---

### 2.2 Style Analysis (css package)

**Purpose**: Load CSS, compute element styles, handle cascade and inheritance.

**Key Classes:**

1. **DOMAnalyzer** (2411 LOC - most complex class)
   - Responsibility: Central CSS processing coordinator
   - Key Operations:
     - Load and parse CSS stylesheets
     - Convert HTML attributes to CSS (e.g., `<font>` tags)
     - Compute effective styles for each element
     - Handle CSS cascade (user-agent → user → author)
     - Manage media queries
   
   **Key Methods:**
   ```java
   - attributesToStyles()           // HTML→CSS conversion
   - addStyleSheet(url, css, origin) // Add stylesheet
   - getStyleSheets()               // Load linked stylesheets
   - getElementStyle(element)       // Get computed style
   ```

2. **CSSNorm**
   - Static methods for standard stylesheets
   - `stdStyleSheet()`: HTML default styles
   - `userStyleSheet()`: Additional defaults
   - `formsStyleSheet()`: Form element styles

3. **FontDecoder** / **FontTable**
   - Font resolution and management
   - Handle @font-face rules

4. **CSSUnits**
   - Unit conversion (px, em, rem, %, pt, etc.)
   - Context-dependent calculations

**Functional Elisp Mapping:**
```elisp
(defun etml-css-normalize-html-attributes (dom-node)
  "Convert HTML presentation attributes to CSS properties."
  (let ((tag (dom-tag dom-node))
        (attrs (dom-attributes dom-node)))
    ;; Return new node with added :style property
    (etml--merge-inline-styles 
     dom-node
     (etml--html-attrs-to-css tag attrs))))

(defun etml-css-parse-stylesheet (css-string base-url)
  "Parse CSS string into rule list."
  ;; Use built-in css-mode parsing or external library
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

**Implementation Strategy:**
- **CSS Parsing**: Consider using external CSS parser or implement basic parser
- **Selector Matching**: Implement CSS selector engine (use `dom.el` helpers)
- **Cascade Algorithm**: Three ordered lists (user-agent, user, author), merge by specificity
- **Inheritance**: Define inheritable properties, propagate down tree

**Complexity Warning**: This is the most complex subsystem. Start with minimal CSS support:
1. Basic selectors (tag, class, id, descendant)
2. Core properties (display, position, width, height, margin, padding)
3. Colors and fonts
4. Gradually add advanced features

---

### 2.3 Layout Engine (layout package) - THE CORE

**Purpose**: Transform styled DOM into positioned box tree with computed dimensions.

This is **63% of the codebase** and the most critical component.

#### 2.3.1 Box Model Hierarchy

**Class Hierarchy:**
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

**Key Properties of Box:**
```java
- Node node;              // DOM node
- Rectangle bounds;       // Position relative to parent
- Rectangle absbounds;    // Absolute position
- ElementBox parent;      // Parent box
- ElementBox cbox;        // Containing block
- VisualContext ctx;      // Font, color, etc.
- boolean isblock;        // Block vs inline
- boolean displayed;      // display !== none
- boolean visible;        // visibility !== hidden
```

**Functional Elisp Mapping:**
```elisp
(cl-defstruct etml-box
  "Base box structure."
  node           ; DOM node reference
  bounds         ; (x y width height) relative to parent
  abs-bounds     ; (x y width height) absolute
  parent         ; Parent etml-box
  containing-block ; Containing block for positioning
  context        ; etml-visual-context
  is-block       ; t for block, nil for inline
  displayed      ; t if display !== none
  visible        ; t if visibility !== hidden
  children)      ; List of child boxes

(cl-defstruct (etml-element-box (:include etml-box))
  "Element box with style."
  element        ; DOM element
  style          ; Computed CSS properties (plist)
  margin         ; (top right bottom left)
  border         ; (top right bottom left)
  padding        ; (top right bottom left))

(cl-defstruct (etml-block-box (:include etml-element-box))
  "Block-level box."
  content-width
  content-height
  floats)        ; List of floated boxes

(cl-defstruct (etml-inline-box (:include etml-element-box))
  "Inline box."
  line-boxes)    ; List of line boxes

(cl-defstruct (etml-text-box (:include etml-box))
  "Text content box."
  text           ; String content
  font-metrics)  ; Font size, line-height, etc.

;; Pure function to create boxes
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
     
     ;; ... handle other display types
     )))
```

#### 2.3.2 BoxFactory - Box Tree Construction

**Purpose**: Transform DOM tree into Box tree.

**Key Algorithm:**
```java
1. Traverse DOM tree (depth-first)
2. For each element:
   a. Get computed style
   b. Create appropriate Box subclass based on display property
   c. Handle anonymous boxes (wrap inline→block transitions)
   d. Handle pseudo-elements (::before, ::after)
   e. Process children recursively
3. Handle special cases:
   - Tables: insert anonymous table wrappers
   - Lists: create list markers
   - Replaced elements: load content
```

**Functional Elisp Mapping:**
```elisp
(defun etml-build-box-tree (dom-tree stylesheets viewport-size)
  "Build box tree from DOM-TREE with applied STYLESHEETS.
Returns root etml-viewport box."
  (let* ((root-element (etml--find-root-element dom-tree))
         (viewport (etml--create-viewport viewport-size))
         (style-map (etml--compute-all-styles dom-tree stylesheets)))
    ;; Recursively build tree
    (etml--build-box-subtree root-element viewport style-map)
    viewport))

(defun etml--build-box-subtree (dom-node parent-box style-map)
  "Recursively build box subtree for DOM-NODE."
  (let* ((style (gethash dom-node style-map))
         (display (plist-get style :display)))
    (cond
     ;; Skip display:none
     ((eq display 'none) nil)
     
     ;; Create box and process children
     (t
      (let ((box (etml-create-box dom-node style parent-box)))
        ;; Process children
        (dolist (child (dom-children dom-node))
          (when-let ((child-box (etml--build-box-subtree 
                                  child box style-map)))
            (push child-box (etml-box-children box))))
        ;; Return box with children
        box)))))
```

#### 2.3.3 Layout Computation - Width/Height Calculation

**This is the most complex algorithm.** It implements CSS 2.1 box model and positioning.

**Key Phases:**

1. **Width Calculation** (top-down)
   ```java
   - Available width flows from parent to children
   - Block width: resolve % relative to parent
   - Inline width: sum of children + padding/border/margin
   ```

2. **Height Calculation** (bottom-up)
   ```java
   - Children are laid out first
   - Parent height = sum of children heights
   - Unless explicit height is set
   ```

3. **Position Calculation**
   ```java
   - Normal flow: sequential positioning
   - Floats: complex algorithm for floating boxes
   - Absolute positioning: relative to containing block
   - Fixed positioning: relative to viewport
   ```

**Functional Elisp Mapping:**
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
         ;; Width calculation
         (width (etml--compute-width style available-width))
         ;; Layout children
         (laid-out-children 
          (etml--layout-children 
           (etml-box-children box) 
           width))
         ;; Height is sum of children (if not explicit)
         (height (or (etml--explicit-height style)
                     (etml--sum-heights laid-out-children)))
         ;; Compute position
         (x (etml--compute-x-position style box))
         (y (etml--compute-y-position style box laid-out-children)))
    ;; Return new box with layout
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
     ;; Explicit width
     ((numberp width-prop) width-prop)
     
     ;; Percentage
     ((and (consp width-prop) (eq (car width-prop) 'percent))
      (* available-width (/ (cdr width-prop) 100.0)))
     
     ;; Auto - fill available space
     ((eq width-prop 'auto)
      (- available-width 
         (etml--sum-horizontal-spacing margin-left margin-right 
                                       padding border))))))

(defun etml--layout-inline-box (box available-width)
  "Layout inline box - complex line breaking algorithm."
  ;; This is very complex - involves:
  ;; 1. Collecting inline content into line boxes
  ;; 2. Breaking lines at available-width
  ;; 3. Vertical alignment within lines
  ;; 4. Handling floats
  (etml--build-line-boxes box available-width))
```

**Critical Implementation Notes:**

1. **Immutability**: All layout functions should return new boxes, not modify existing ones
2. **Two-Pass Algorithm**: 
   - Pass 1: Width (top-down)
   - Pass 2: Height & Position (bottom-up)
3. **Floats are Hard**: Floating layout is one of the most complex CSS features
4. **Line Breaking**: For inline content, implement proper line-breaking algorithm

#### 2.3.4 Viewport - The Root Container

**Purpose**: Top-level container, manages visible area vs. total canvas.

```elisp
(cl-defstruct (etml-viewport (:include etml-block-box))
  "Root viewport box."
  canvas-width    ; Total page width
  canvas-height   ; Total page height
  visible-rect    ; (x y width height) of visible area
  config)         ; Browser configuration

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

#### 2.3.5 Visual Context

**Purpose**: Rendering state (fonts, colors, etc.) that flows through the tree.

```elisp
(cl-defstruct etml-visual-context
  "Visual rendering context."
  font-size       ; Current font size in px
  font-family     ; Font family name
  font-weight     ; normal, bold, etc.
  color           ; Text color
  background      ; Background color
  line-height     ; Line height multiplier
  dpi             ; Screen DPI for unit conversion
  viewport)       ; Reference to viewport

(defun etml-create-context (parent-context style)
  "Create new context inheriting from PARENT-CONTEXT with STYLE."
  (make-etml-visual-context
   :font-size (or (plist-get style :font-size)
                  (etml-visual-context-font-size parent-context))
   :font-family (or (plist-get style :font-family)
                    (etml-visual-context-font-family parent-context))
   ;; ... other properties
   :viewport (etml-visual-context-viewport parent-context)))
```

---

### 2.4 Rendering (render package)

**Purpose**: Convert laid-out box tree to visual output.

**Key Classes:**

1. **BoxRenderer** (interface)
   - `startElement(box)` / `endElement(box)`
   - `renderTextContent(box)`
   
2. **StructuredRenderer** (abstract)
   - Tree traversal logic
   - Visitor pattern for rendering

3. **GraphicsRenderer** (AWT implementation)
   - Draws to Java Graphics2D
   - Handles clipping, transformations

**Functional Elisp Mapping:**
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

**Alternative Rendering Targets:**

1. **Emacs Buffer with Text Properties**
   - Simple, uses built-in text properties
   - Limited visual fidelity (no exact positioning)

2. **SVG Rendering**
   - Emacs can display inline SVG
   - Precise positioning
   ```elisp
   (defun etml-render-to-svg (box-tree width height)
     "Render BOX-TREE to SVG string."
     (concat
      (format "<svg width=\"%d\" height=\"%d\">\n" width height)
      (etml--render-boxes-to-svg (etml-viewport-children box-tree))
      "</svg>"))
   
   (defun etml--render-box-to-svg (box)
     "Render single box to SVG element."
     (let ((bounds (etml-box-bounds box)))
       (format "<rect x=\"%f\" y=\"%f\" width=\"%f\" height=\"%f\" fill=\"%s\"/>\n"
               (nth 0 bounds) (nth 1 bounds)
               (nth 2 bounds) (nth 3 bounds)
               (or (plist-get (etml-element-box-style box) :background-color)
                   "transparent"))))
   ```

3. **Image Rendering with ImageMagick**
   - If Emacs compiled with ImageMagick support
   - Can create actual bitmap images

---

## 3. Key Algorithms in Detail

### 3.1 CSS Cascade Algorithm

**Purpose**: Determine which CSS rule wins when multiple rules match.

**Algorithm:**
```
1. Collect all rules matching the element
2. For each rule, compute specificity:
   - Inline style: specificity = (1,0,0,0)
   - ID: (0,1,0,0)
   - Class/attribute/pseudo-class: (0,0,1,0)
   - Element/pseudo-element: (0,0,0,1)
3. Sort by:
   a. Origin (user-agent < user < author < inline)
   b. !important flag
   c. Specificity
   d. Source order
4. Apply in sorted order (later overwrites earlier)
```

**Elisp Implementation:**
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
    ;; Merge properties in order
    (cl-reduce #'etml--merge-properties
               sorted-rules
               :key #'etml-css-rule-properties
               :initial-value nil)))
```

### 3.2 Box Layout Algorithm (Simplified)

**Purpose**: Position boxes and compute dimensions.

**Main Algorithm:**
```
function layout(box, availableWidth):
  1. Compute width:
     - If explicit width: use it
     - If percentage: compute from availableWidth
     - If auto (block): fill available space
     - If auto (inline): shrink to content
  
  2. Layout children:
     - For block children:
       * Stack vertically
       * Each gets full content width
     - For inline children:
       * Pack horizontally into lines
       * Break lines at availableWidth
  
  3. Compute height:
     - If explicit height: use it
     - Otherwise: sum of children heights
  
  4. Position box:
     - Normal flow: after previous sibling
     - Float: complex float positioning
     - Absolute: relative to containing block
  
  5. Return box with bounds set
```

**Elisp Implementation (Simplified):**
```elisp
(defun etml-layout-box-tree (root-box viewport-width)
  "Layout entire box tree starting from ROOT-BOX."
  ;; Two-pass algorithm
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
    ;; Set width on box
    (setf (nth 2 (etml-box-bounds box)) width)
    ;; Layout children with new width
    (dolist (child (etml-box-children box))
      (etml--layout-pass-width child width))
    box))

(defun etml--layout-pass-height (box)
  "Second pass: compute heights bottom-up."
  ;; First layout all children
  (dolist (child (etml-box-children box))
    (etml--layout-pass-height child))
  ;; Then compute this box's height
  (let ((height (etml--compute-height box)))
    (setf (nth 3 (etml-box-bounds box)) height)
    ;; Position children
    (etml--position-children box)
    box))

(defun etml--compute-height (box)
  "Compute height of box based on children."
  (cond
   ;; Explicit height
   ((when-let ((style (and (etml-element-box-p box)
                           (etml-element-box-style box))))
      (plist-get style :height)))
   
   ;; Text box: use font metrics
   ((etml-text-box-p box)
    (etml--text-height box))
   
   ;; Sum of children
   (t
    (cl-reduce #'+ (etml-box-children box)
               :key (lambda (child) (nth 3 (etml-box-bounds child)))
               :initial-value 0))))

(defun etml--position-children (parent-box)
  "Set x,y positions for children of PARENT-BOX."
  (let ((y 0))  ; Running Y position
    (dolist (child (etml-box-children parent-box))
      (let ((bounds (etml-box-bounds child)))
        ;; Set Y position
        (setf (nth 1 bounds) y)
        ;; Advance Y
        (cl-incf y (nth 3 bounds))))))  ; height
```

### 3.3 Selector Matching Algorithm

**Purpose**: Determine if CSS selector matches an element.

```elisp
(defun etml-css-selector-matches-p (selector element dom-tree)
  "Return t if SELECTOR matches ELEMENT in DOM-TREE."
  (let ((parts (etml--parse-selector selector)))
    (etml--match-selector-parts parts element dom-tree)))

(defun etml--match-selector-parts (parts element dom-tree)
  "Match compound selector PARTS against ELEMENT."
  (cond
   ;; Empty selector - match
   ((null parts) t)
   
   ;; Descendant combinator: 'div p'
   ((eq (car parts) 'descendant)
    (and (etml--match-simple-selector (cadr parts) element)
         (etml--has-ancestor-matching (cddr parts) element dom-tree)))
   
   ;; Child combinator: 'div > p'
   ((eq (car parts) 'child)
    (and (etml--match-simple-selector (cadr parts) element)
         (when-let ((parent (etml--get-parent element dom-tree)))
           (etml--match-selector-parts (cddr parts) parent dom-tree))))
   
   ;; Simple selector: 'div', '.class', '#id'
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

## 4. Functional Programming Adaptation

### 4.1 Eliminating Global State

**Java Issues:**
- Mutable box trees
- Shared caches (ImageCache, FontTable)
- Global configuration (BrowserConfig)

**Elisp Solutions:**

1. **Immutable Data Structures**
   ```elisp
   ;; Instead of mutating boxes, return new boxes
   (defun etml-set-box-bounds (box new-bounds)
     "Return new box with NEW-BOUNDS."
     (let ((new-box (copy-etml-box box)))
       (setf (etml-box-bounds new-box) new-bounds)
       new-box))
   ```

2. **Thread State Through Functions**
   ```elisp
   ;; Instead of global config, pass as parameter
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

3. **Caches as Data, Not Side Effects**
   ```elisp
   ;; Return cache as part of result
   (defun etml-load-image (url cache)
     "Load image from URL using CACHE.
   Returns (image . new-cache)."
     (or (gethash url cache)
         (let ((image (etml--fetch-image url)))
           (cons image (puthash url image (copy-hash-table cache))))))
   
   ;; Use with multiple values
   (cl-multiple-value-bind (image new-cache)
       (etml-load-image "http://..." image-cache)
     ;; Use image, thread new-cache forward
     ...)
   ```

### 4.2 Pure Function Design Patterns

**Pattern 1: Transform, Don't Mutate**
```elisp
;; Bad (mutable)
(defun bad-layout-box (box)
  (setf (etml-box-width box) 100)
  box)

;; Good (immutable)
(defun etml-layout-box (box)
  (etml--copy-box box :width 100))
```

**Pattern 2: Explicit Context Passing**
```elisp
;; Pass all dependencies explicitly
(defun etml-layout-element (element style context parent-box)
  "Layout ELEMENT with STYLE in CONTEXT, child of PARENT-BOX."
  ;; All inputs are parameters, no globals
  ...)
```

**Pattern 3: Nested Contexts**
```elisp
;; Context flows down tree
(defun etml-render-tree (box context)
  (let ((child-context (etml-create-child-context context box)))
    (dolist (child (etml-box-children box))
      (etml-render-tree child child-context))))
```

---

## 5. Implementation Roadmap

### Phase 1: Foundation (2-3 weeks)

**Goal**: Basic HTML parsing and DOM handling.

```elisp
;; Deliverables:
(etml-parse-html STRING)              ; → DOM tree
(etml-dom-query DOM SELECTOR)         ; → matching nodes
(etml-dom-traverse DOM FN)            ; Tree traversal
```

**Files to create:**
- `etml-dom.el` - DOM utilities
- `etml-html-parser.el` - HTML parsing wrapper

**Complexity**: Low
**Lines of code**: ~500

### Phase 2: CSS Basics (3-4 weeks)

**Goal**: Parse and match basic CSS.

```elisp
;; Deliverables:
(etml-css-parse STYLESHEET)           ; → rule list
(etml-css-match-rules RULES ELEMENT)  ; → matching rules
(etml-css-compute-style ELEMENT RULES); → style plist
```

**Files to create:**
- `etml-css-parser.el` - CSS parsing
- `etml-css-selector.el` - Selector matching
- `etml-css-cascade.el` - Cascade algorithm
- `etml-css-properties.el` - Property definitions

**Complexity**: Medium-High
**Lines of code**: ~2000

**Subset to implement first:**
- Selectors: tag, class, id, descendant, child
- Properties: display, width, height, margin, padding, border
- Units: px, %, em
- Colors: named colors, hex

### Phase 3: Box Model (4-5 weeks)

**Goal**: Create box tree and compute layout.

```elisp
;; Deliverables:
(etml-build-box-tree DOM STYLES)      ; → box tree
(etml-layout-boxes BOX-TREE SIZE)     ; → laid out tree
```

**Files to create:**
- `etml-box.el` - Box structures
- `etml-box-factory.el` - Box creation
- `etml-layout-block.el` - Block layout
- `etml-layout-inline.el` - Inline layout (simplified)
- `etml-layout-position.el` - Positioning

**Complexity**: High
**Lines of code**: ~3000

**Simplifications for v1:**
- No floats
- No absolute/fixed positioning
- No tables
- Simplified inline layout (no line breaking)

### Phase 4: Rendering (2-3 weeks)

**Goal**: Display laid-out boxes.

```elisp
;; Deliverables:
(etml-render-to-buffer BOX-TREE)      ; → buffer with text props
(etml-render-to-svg BOX-TREE)         ; → SVG string
```

**Files to create:**
- `etml-render-buffer.el` - Buffer rendering
- `etml-render-svg.el` - SVG rendering

**Complexity**: Medium
**Lines of code**: ~800

### Phase 5: Advanced Features (ongoing)

**Add incrementally:**
1. Floats
2. Absolute/fixed positioning  
3. Tables
4. Proper line breaking
5. More CSS properties (fonts, backgrounds, borders)
6. Images
7. Forms
8. JavaScript hooks (maybe)

---

## 6. Elisp Function Naming Convention

### 6.1 Public API (prefix: `etml-`)

**Document Processing:**
```elisp
(etml-parse-html html-string)
(etml-parse-url url)
(etml-render-document url &optional config)
```

**DOM Manipulation:**
```elisp
(etml-dom-query dom selector)
(etml-dom-traverse dom function)
(etml-dom-get-attribute node attr)
(etml-dom-get-text node)
```

**CSS Operations:**
```elisp
(etml-css-parse stylesheet-string)
(etml-css-compute-style element rules)
(etml-css-match-selector selector element)
```

**Box Model:**
```elisp
(etml-create-box-tree dom styles viewport-size)
(etml-layout-box-tree box-tree)
(etml-box-bounds box)
(etml-box-children box)
```

**Rendering:**
```elisp
(etml-render-to-buffer box-tree buffer)
(etml-render-to-svg box-tree)
(etml-render-to-string box-tree)
```

### 6.2 Internal Functions (prefix: `etml--`)

**Use double dash for internals:**
```elisp
(etml--parse-selector selector)
(etml--match-simple-selector selector element)
(etml--compute-width style available-width)
(etml--layout-block-box box width)
(etml--position-children parent-box)
```

### 6.3 Type Constructors (prefix: `make-etml-`)

**Generated by cl-defstruct:**
```elisp
(make-etml-box ...)
(make-etml-block-box ...)
(make-etml-visual-context ...)
```

### 6.4 Type Predicates (suffix: `-p`)

**Generated by cl-defstruct:**
```elisp
(etml-box-p obj)
(etml-block-box-p obj)
(etml-element-box-p obj)
```

### 6.5 Constants (prefix: `etml-` suffix: none)

```elisp
(defconst etml-default-font-size 16)
(defconst etml-default-line-height 1.2)
(defconst etml-named-colors '((red . "#FF0000") ...))
```

### 6.6 Configuration (prefix: `etml-` suffix: none)

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

## 7. Module Structure

**Recommended file organization:**

```
etml/
├── etml.el                  ; Main entry point, autoloads
├── etml-core.el             ; Core types and utilities
├── etml-dom.el              ; DOM utilities
├── etml-html-parser.el      ; HTML parsing
├── etml-css-parser.el       ; CSS parsing
├── etml-css-selector.el     ; Selector matching
├── etml-css-cascade.el      ; Cascade algorithm  
├── etml-css-properties.el   ; Property definitions
├── etml-css-units.el        ; Unit conversion
├── etml-box.el              ; Box structures
├── etml-box-factory.el      ; Box tree creation
├── etml-layout.el           ; Layout entry point
├── etml-layout-block.el     ; Block layout
├── etml-layout-inline.el    ; Inline layout
├── etml-layout-position.el  ; Positioning
├── etml-render.el           ; Rendering entry point
├── etml-render-buffer.el    ; Buffer rendering
├── etml-render-svg.el       ; SVG rendering
└── etml-test.el             ; Tests
```

**Main entry point (`etml.el`):**
```elisp
;;; etml.el --- HTML/CSS rendering engine for Emacs -*- lexical-binding: t -*-

;; Copyright (C) 2025 Your Name

;; Author: Your Name <your.email@example.com>
;; Version: 0.1.0
;; Package-Requires: ((emacs "27.1") (cl-lib "0.5"))
;; Keywords: html, css, rendering
;; URL: https://github.com/yourusername/etml

;;; Commentary:

;; ETML is a pure Emacs Lisp implementation of an HTML/CSS rendering
;; engine, inspired by the CSSBox library.  It provides functions to
;; parse HTML, apply CSS styles, compute layout, and render to various
;; targets.

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

## 8. Testing Strategy

### 8.1 Unit Tests

**Test each module independently:**

```elisp
;;; etml-test.el

(require 'ert)
(require 'etml)

;; CSS Parser tests
(ert-deftest etml-test-css-parse-simple ()
  "Test parsing simple CSS rule."
  (let ((css "div { color: red; }")
        (expected '((:selector "div" 
                     :properties (:color "#FF0000")))))
    (should (equal (etml-css-parse css) expected))))

;; Selector matching tests
(ert-deftest etml-test-selector-match-tag ()
  "Test tag selector matching."
  (let ((dom (etml-parse-html "<div><p>Test</p></div>"))
        (p-element (car (etml-dom-query dom "p"))))
    (should (etml-css-match-selector "p" p-element dom))
    (should-not (etml-css-match-selector "div" p-element dom))))

;; Layout tests
(ert-deftest etml-test-block-width ()
  "Test block width computation."
  (let* ((box (make-etml-block-box 
               :style '(:width 100 :margin-left 10 :margin-right 10)))
         (laid-out (etml--compute-block-width box 200)))
    (should (= (nth 2 (etml-box-bounds laid-out)) 100))))

;; Integration test
(ert-deftest etml-test-render-simple-document ()
  "Test rendering simple HTML document."
  (let* ((html "<html><body><p>Hello, world!</p></body></html>")
         (dom (etml-parse-html html))
         (box-tree (etml-create-box-tree dom nil '(800 600))))
    (should (etml-viewport-p box-tree))
    (should (> (length (etml-box-children box-tree)) 0))))
```

### 8.2 Visual Regression Tests

**Compare rendered output:**

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

## 9. Key Challenges and Solutions

### Challenge 1: CSS Parsing Complexity

**Problem**: CSS has complex syntax (media queries, pseudo-classes, etc.)

**Solutions:**
1. **Use existing library**: Consider porting a simple CSS parser from JS
2. **Subset approach**: Start with basic CSS2.1, add features incrementally
3. **Regex-based parser**: For initial version, regex can handle simple rules

### Challenge 2: Layout Algorithm Complexity

**Problem**: CSS layout (especially floats, positioning) is very complex

**Solutions:**
1. **Start simple**: Normal flow only (no floats, no absolute positioning)
2. **Two-pass algorithm**: Width top-down, height bottom-up
3. **Reference implementation**: Use CSSBox Java code as reference
4. **Test cases**: Use W3C CSS test suite for validation

### Challenge 3: Font Metrics in Emacs

**Problem**: Need accurate font metrics for text layout

**Solutions:**
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

### Challenge 4: Immutability Performance

**Problem**: Copying box trees is expensive

**Solutions:**
1. **Structural sharing**: Use `copy-sequence` only for modified parts
2. **Lazy evaluation**: Compute layout on-demand
3. **Memoization**: Cache layout results
4. **Acceptable trade-off**: Correctness > performance for v1

---

## 10. Dependencies and Libraries

### 10.1 Built-in Emacs Libraries

**Already available:**
- `dom.el` - DOM manipulation
- `libxml` - HTML/XML parsing
- `url` - HTTP fetching
- `svg` - SVG generation

### 10.2 External Dependencies (Optional)

**Consider using:**
- `request.el` - Better HTTP library
- `s.el` - String manipulation utilities
- `dash.el` - List manipulation utilities
- `ht.el` - Hash table utilities

### 10.3 Dependency Management

```elisp
;; In etml.el header:
;; Package-Requires: ((emacs "27.1") (cl-lib "0.5"))
```

**Minimal dependencies approach**: Use only built-in Emacs libraries for core functionality.

---

## 11. Performance Considerations

### 11.1 Expected Performance

**Baseline (simple document):**
- Parse: < 100ms
- Style computation: < 200ms
- Layout: < 300ms
- Render: < 500ms
- **Total: ~1 second for typical page**

### 11.2 Optimization Strategies

1. **Lazy evaluation**
   ```elisp
   (defun etml-box-get-bounds (box)
     "Get bounds, computing lazily if needed."
     (or (etml-box--cached-bounds box)
         (setf (etml-box--cached-bounds box)
               (etml--compute-bounds box))))
   ```

2. **Memoization**
   ```elisp
   (defun etml-css-compute-style-memoized (element rules)
     "Compute style with memoization."
     (or (gethash element etml--style-cache)
         (puthash element
                  (etml-css-compute-style element rules)
                  etml--style-cache)))
   ```

3. **Incremental updates** (future)
   - Only re-layout changed subtrees
   - Dirty flag propagation

### 11.3 Memory Usage

**Estimated memory for typical page:**
- DOM tree: ~50 KB
- Style map: ~100 KB
- Box tree: ~200 KB
- **Total: ~350 KB per page**

---

## 12. Example Usage Patterns

### Pattern 1: Render URL to Buffer

```elisp
(etml-render-url "https://example.com")
;; Opens new buffer with rendered content
```

### Pattern 2: Render HTML String

```elisp
(let* ((html "<html><body><h1>Title</h1><p>Content</p></body></html>")
       (dom (etml-parse-html html))
       (config (list :viewport-size '(800 600)
                     :font-size 16))
       (box-tree (etml-render-document-internal dom nil config)))
  (etml-render-to-buffer box-tree (current-buffer)))
```

### Pattern 3: Custom Rendering

```elisp
(defun my-custom-renderer (url)
  "Custom rendering with SVG output."
  (let* ((html (etml--fetch-url url))
         (dom (etml-parse-html html))
         (box-tree (etml-create-box-tree dom nil '(1200 800)))
         (laid-out (etml-layout-box-tree box-tree 1200))
         (svg (etml-render-to-svg laid-out)))
    ;; Insert SVG into buffer
    (insert-image (create-image svg 'svg t))))
```

### Pattern 4: Extract Structured Data

```elisp
(defun etml-extract-links (url)
  "Extract all links from page at URL."
  (let* ((html (etml--fetch-url url))
         (dom (etml-parse-html html))
         (links (etml-dom-query dom "a[href]")))
    (mapcar (lambda (link) (dom-attr link 'href)) links)))
```

---

## 13. Comparison with CSSBox

### Similarities
- Same overall architecture (parse → style → layout → render)
- Similar box model hierarchy
- Two-pass layout algorithm
- Factory pattern for box creation

### Differences

| Aspect | CSSBox (Java) | ETML (Elisp) |
|--------|---------------|--------------|
| **Language** | Java (OOP) | Elisp (Functional) |
| **Mutability** | Mutable boxes | Immutable (or minimal mutation) |
| **State** | Global caches | Threaded through functions |
| **Inheritance** | Class hierarchy | Struct composition |
| **Rendering** | Java2D Graphics | Emacs text properties / SVG |
| **Dependencies** | jStyleParser, NekoHTML | Built-in libxml, dom.el |
| **Target** | General purpose | Emacs-specific |
| **Performance** | Fast (~100ms for complex pages) | Slower (~1s acceptable) |
| **Completeness** | Full CSS 2.1 + CSS3 subset | Minimal CSS subset initially |

### Scope Reduction

**CSSBox features NOT needed for v1:**
- Full CSS3 support (gradients, transforms, etc.)
- JavaScript execution
- Form interaction
- SVG inline rendering
- PDF output
- Detailed font metrics (use Emacs face system)
- Network caching (Emacs handles this)
- Threading (Emacs is single-threaded)

---

## 14. References and Resources

### Documentation
- **CSSBox Manual**: `/doc/manual/manual.html`
- **CSSBox API**: Generated JavaDoc
- **CSS 2.1 Spec**: https://www.w3.org/TR/CSS21/
- **W3C CSS Test Suite**: https://test.csswg.org/

### Key CSSBox Classes to Study
1. `DOMAnalyzer.java` - CSS processing
2. `BoxFactory.java` - Box creation
3. `ElementBox.java` - Base element box
4. `BlockBox.java` - Block layout
5. `Viewport.java` - Root container
6. `Engine.java` - Main coordinator

### Emacs Resources
- **Emacs Lisp Manual**: `(info "(elisp)")` 
- **dom.el**: Built-in DOM utilities
- **Text Properties**: `(info "(elisp) Text Properties")`
- **SVG**: `(info "(elisp) SVG Images")`

### Testing Resources
- **W3C CSS Test Suite**: Comprehensive test cases
- **Acid Tests**: Acid1, Acid2 for browser compatibility
- **Test HTML**: Simple test cases in `/doc/examples/`

---

## 15. Quick Start Development Guide

### Step 1: Setup Project

```bash
mkdir etml
cd etml
git init
touch etml.el etml-core.el etml-dom.el
```

### Step 2: Define Core Types

```elisp
;; etml-core.el
(cl-defstruct etml-box ...)
(cl-defstruct (etml-element-box (:include etml-box)) ...)
```

### Step 3: Implement HTML Parser

```elisp
;; etml-html-parser.el
(defun etml-parse-html (html-string)
  (with-temp-buffer
    (insert html-string)
    (libxml-parse-html-region (point-min) (point-max))))
```

### Step 4: Basic CSS (Start Here)

```elisp
;; etml-css-parser.el
(defun etml-css-parse (css-string)
  ;; Parse "selector { prop: value; }" format
  ...)
```

### Step 5: Simple Layout

```elisp
;; etml-layout.el
(defun etml-layout-box-tree (box-tree width)
  ;; Implement basic block stacking
  ...)
```

### Step 6: Buffer Rendering

```elisp
;; etml-render-buffer.el
(defun etml-render-to-buffer (box-tree buffer)
  ;; Insert text with faces
  ...)
```

### Step 7: Test!

```elisp
(ert-run-tests-interactively "etml-")
```

---

## 16. Conclusion

### Summary of Key Points

1. **CSSBox is a well-architected HTML/CSS rendering engine** with clear separation of concerns
2. **The core is the layout engine** (63% of code) - focus implementation effort here
3. **CSS cascade and selector matching** are complex but well-defined algorithms
4. **Functional adaptation** is feasible with immutable data and explicit state passing
5. **Start small**: Basic HTML + minimal CSS → expand incrementally
6. **Use Emacs built-ins**: libxml, dom.el, text properties, SVG

### Estimated Total Effort

**For minimal working implementation:**
- **Core engine**: 8-10 weeks (CSS + layout + rendering)
- **Advanced features**: Ongoing (floats, tables, forms, etc.)
- **Total lines**: ~6,000-8,000 LOC Elisp for v1

### Success Criteria for v1

- [ ] Parse simple HTML documents
- [ ] Apply basic CSS (tag, class, id selectors)
- [ ] Compute layout for block elements (no floats)
- [ ] Render to Emacs buffer with text properties
- [ ] Handle basic properties (width, height, margin, padding, color, font-size)
- [ ] Functional API (no global state)
- [ ] Test suite with 50+ test cases

### Next Steps

1. **Start with `etml-core.el`**: Define box structures
2. **Build `etml-html-parser.el`**: Wrap libxml
3. **Implement `etml-css-selector.el`**: Selector matching (most important)
4. **Develop `etml-layout-block.el`**: Simple block stacking
5. **Create `etml-render-buffer.el`**: Text property rendering
6. **Test continuously**: Add test for each feature

### Final Recommendation

**This is a substantial project (~2-3 months for v1).** The CSSBox codebase is mature and well-structured, making it an excellent reference. The functional adaptation to Elisp is straightforward but requires disciplined immutability. Start with the smallest possible subset and expand incrementally. Focus on correctness over performance initially.

Good luck with the reimplementation! 🚀

---

## Appendix A: Function Name Index

**Complete list of recommended public functions:**

### Document Processing
```elisp
etml-parse-html
etml-parse-url
etml-render-document
etml-render-url
```

### DOM Operations
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

### CSS Operations
```elisp
etml-css-parse
etml-css-parse-inline
etml-css-compute-style
etml-css-compute-specificity
etml-css-match-selector
etml-css-load-stylesheet
etml-css-unit-to-px
```

### Box Model
```elisp
etml-create-box-tree
etml-box-bounds
etml-box-abs-bounds
etml-box-children
etml-box-parent
etml-box-style
etml-box-is-block-p
```

### Layout
```elisp
etml-layout-box-tree
etml-layout-box
etml-compute-dimensions
etml-compute-position
```

### Rendering
```elisp
etml-render-to-buffer
etml-render-to-svg
etml-render-to-string
etml-render-box
```

### Configuration
```elisp
etml-default-config
etml-create-config
etml-config-set
etml-config-get
```

### Utilities
```elisp
etml-color-parse
etml-color-to-hex
etml-font-parse
etml-unit-parse
```

---

**End of Architecture Analysis**

*Document Version: 1.0*
*Date: 2025-11-03*
*Total Pages: ~40*
*Word Count: ~8,500*
