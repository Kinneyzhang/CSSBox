# CSSBox 类依赖关系详细分析
本文档详细列出了 CSSBox 项目中所有 Java 类的信息和依赖关系。
## 总览
- **总类数**: 91
- **包数量**: 8

## 目录
1. [org.fit.cssbox.awt](#orgfitcssboxawt)
2. [org.fit.cssbox.css](#orgfitcssboxcss)
3. [org.fit.cssbox.demo](#orgfitcssboxdemo)
4. [org.fit.cssbox.io](#orgfitcssboxio)
5. [org.fit.cssbox.layout](#orgfitcssboxlayout)
6. [org.fit.cssbox.misc](#orgfitcssboxmisc)
7. [org.fit.cssbox.render](#orgfitcssboxrender)
8. [org.fit.cssbox.testing](#orgfitcssboxtesting)

---

## org.fit.cssbox.awt
**类数量**: 9

### BackgroundBitmap

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/BackgroundBitmap.java`
- **继承**: `ElementBackground`
- **内部依赖**: 9 个类
  - `org.fit.cssbox.layout.ElementBox`
  - `org.fit.cssbox.layout.Rectangle`
  - `org.fit.cssbox.render.BackgroundImageGradient`
  - `org.fit.cssbox.render.BackgroundImageImage`
  - `org.fit.cssbox.render.BackgroundRepeater`
  - ... 和其他 4 个
- **主要公共方法** (3 个):
  - `BufferedImage getBufferedImage(...)`
  - `void addBackgroundImage(...)`
  - `void addBackgroundImage(...)`

### BitmapImage

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/BitmapImage.java`
- **实现接口**: `ContentImage`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.layout.ContentImage`
- **主要公共方法** (4 个):
  - `URL getUrl(...)`
  - `BufferedImage getBufferedImage(...)`
  - `float getWidth(...)`
  - `float getHeight(...)`

### BrowserCanvas

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/BrowserCanvas.java`
- **继承**: `JPanel`
- **内部依赖**: 5 个类
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.layout.BrowserConfig`
  - `org.fit.cssbox.layout.Dimension`
  - `org.fit.cssbox.layout.Engine`
  - `org.fit.cssbox.layout.Rectangle`
- **主要公共方法** (8 个):
  - `GraphicsEngine getEngine(...)`
  - `void paintComponent(...)`
  - `void createLayout(...)`
  - `void createLayout(...)`
  - `void createLayout(...)`
  - `void redrawBoxes(...)`
  - `void setConfig(...)`
  - `BrowserConfig getConfig(...)`

### CSSStroke

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/CSSStroke.java`
- **实现接口**: `Stroke`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.misc.Coords`
- **外部依赖**: `cz.vutbr`, `org.slf4j`
- **主要公共方法** (1 个):
  - `Shape createStrokedShape(...)`

### GraphicsEngine

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/GraphicsEngine.java`
- **继承**: `Engine`
- **内部依赖**: 8 个类
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.css.FontTable`
  - `org.fit.cssbox.layout.BrowserConfig`
  - `org.fit.cssbox.layout.Dimension`
  - `org.fit.cssbox.layout.Engine`
  - ... 和其他 3 个
- **外部依赖**: `org.w3c`
- **主要公共方法** (8 个):
  - `Graphics2D getImageGraphics(...)`
  - `BufferedImage getImage(...)`
  - `void setImage(...)`
  - `boolean isUseFractionalMetrics(...)`
  - `void setUseFractionalMetrics(...)`
  - `boolean isUseKerning(...)`
  - `void setUseKerning(...)`
  - `BoxRenderer getRenderer(...)`

### GraphicsImageLoader

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/GraphicsImageLoader.java`
- **实现接口**: `ImageLoader`
- **内部依赖**: 6 个类
  - `org.fit.cssbox.io.ContentObserver`
  - `org.fit.cssbox.io.DocumentSource`
  - `org.fit.cssbox.layout.ContentImage`
  - `org.fit.cssbox.layout.ImageCache`
  - `org.fit.cssbox.layout.ImageLoader`
  - ... 和其他 1 个
- **外部依赖**: `org.slf4j`
- **主要公共方法** (2 个):
  - `ContentImage loadImage(...)`
  - `ContentImage loadImageFromSource(...)`

### GraphicsRenderer

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/GraphicsRenderer.java`
- **继承**: `StructuredRenderer`
- **内部依赖**: 20 个类
  - `org.fit.cssbox.css.BackgroundDecoder`
  - `org.fit.cssbox.layout.BackgroundImage`
  - `org.fit.cssbox.layout.BlockBox`
  - `org.fit.cssbox.layout.Box`
  - `org.fit.cssbox.layout.ContentImage`
  - ... 和其他 15 个
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (8 个):
  - `void clearCanvas(...)`
  - `void startElementContents(...)`
  - `void finishElementContents(...)`
  - `void renderElementBackground(...)`
  - `void renderMarker(...)`
  - `void renderTextContent(...)`
  - `void renderReplacedContent(...)`
  - `void close(...)`

### GraphicsVisualContext

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/GraphicsVisualContext.java`
- **继承**: `VisualContext`
- **内部依赖**: 9 个类
  - `org.fit.cssbox.css.CSSUnits`
  - `org.fit.cssbox.css.FontDecoder`
  - `org.fit.cssbox.css.FontSpec`
  - `org.fit.cssbox.css.FontTable`
  - `org.fit.cssbox.io.DocumentSource`
  - ... 和其他 4 个
- **外部依赖**: `cz.vutbr`, `org.fit`
- **主要公共方法** (16 个):
  - `VisualContext create(...)`
  - `void copyVisualContext(...)`
  - `void update(...)`
  - `float getEx(...)`
  - `float getCh(...)`
  - `void updateGraphics(...)`
  - `float stringWidth(...)`
  - `void setCurrentFont(...)`
  - `String getFontFamily(...)`
  - `FontInfo getFontInfo(...)`
  - ... 和其他 6 个方法

### Transform

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/awt/Transform.java`
- **内部依赖**: 3 个类
  - `org.fit.cssbox.layout.CSSDecoder`
  - `org.fit.cssbox.layout.ElementBox`
  - `org.fit.cssbox.layout.Rectangle`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (1 个):
  - `AffineTransform createTransform(...)`

---

## org.fit.cssbox.css
**类数量**: 11

### BackgroundDecoder

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/BackgroundDecoder.java`
- **内部依赖**: 11 个类
  - `org.fit.cssbox.layout.BackgroundImage`
  - `org.fit.cssbox.layout.CSSDecoder`
  - `org.fit.cssbox.layout.ElementBox`
  - `org.fit.cssbox.layout.Rectangle`
  - `org.fit.cssbox.layout.VisualContext`
  - ... 和其他 6 个
- **外部依赖**: `cz.vutbr`, `org.fit`, `org.slf4j`
- **主要公共方法** (4 个):
  - `ElementBox getOwner(...)`
  - `Color getBgcolor(...)`
  - `List<BackgroundImage> getBackgroundImages(...)`
  - `boolean isBackgroundEmpty(...)`

### CSSNorm

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/CSSNorm.java`
- **主要公共方法** (3 个):
  - `String stdStyleSheet(...)`
  - `String userStyleSheet(...)`
  - `String formsStyleSheet(...)`

### CSSUnits

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/CSSUnits.java`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (4 个):
  - `float pixels(...)`
  - `float points(...)`
  - `float convertFontSize(...)`
  - `float convertBorderWidth(...)`

### Counters

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/Counters.java`
- **继承**: `HashMap<String, Integer>`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (6 个):
  - `Counters getScope(...)`
  - `int getCounter(...)`
  - `List<Integer> getCounters(...)`
  - `void resetCounter(...)`
  - `void incrementCounter(...)`
  - `void applyStyle(...)`

### DOMAnalyzer

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/DOMAnalyzer.java`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `String getDefaultEncoding(...)`
  - `void setDefaultEncoding(...)`
  - `String getMedia(...)`
  - `void setMedia(...)`
  - `MediaSpec getMediaSpec(...)`
  - `void setMediaSpec(...)`
  - `Element getRoot(...)`
  - `Element getHead(...)`
  - `Element getBody(...)`
  - `String getDocumentBase(...)`
  - ... 和其他 10 个方法

### FontDecoder

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/FontDecoder.java`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.io.DocumentSource`
- **主要公共方法** (3 个):
  - `void registerFont(...)`
  - `String findRegisteredFont(...)`
  - `Font decodeFont(...)`

### FontSpec

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/FontSpec.java`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (8 个):
  - `String getFamily(...)`
  - `void setFamily(...)`
  - `void setWeight(...)`
  - `void setStyle(...)`
  - `int hashCode(...)`
  - `boolean equals(...)`
  - `int match(...)`
  - `boolean representsBold(...)`

### FontTable

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/FontTable.java`
- **继承**: `LinkedHashMap<FontSpec, List<RuleFontFace.Source>>`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (1 个):
  - `List<RuleFontFace.Source> findBestMatch(...)`

### HTMLNorm

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/HTMLNorm.java`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (5 个):
  - `String getAttribute(...)`
  - `void attributesToStyles(...)`
  - `float computeAttributeLength(...)`
  - `TermLengthOrPercent createLengthOrPercent(...)`
  - `void normalizeHTMLTree(...)`

### NormalOutput

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/css/NormalOutput.java`
- **继承**: `Output`
- **外部依赖**: `org.w3c`
- **主要公共方法** (2 个):
  - `void dumpTo(...)`
  - `void dumpTo(...)`

### Output

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/css/Output.java`
- **外部依赖**: `org.w3c`

---

## org.fit.cssbox.demo
**类数量**: 6

### BoxBrowser

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/BoxBrowser.java`
- **内部依赖**: 18 个类
  - `org.fit.cssbox.awt.BrowserCanvas`
  - `org.fit.cssbox.css.BackgroundDecoder`
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.CSSUnits`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - ... 和其他 13 个
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (20 个):
  - `BrowserConfig getConfig(...)`
  - `URL displayURL(...)`
  - `void mouseClicked(...)`
  - `void mousePressed(...)`
  - `void mouseReleased(...)`
  - `void mouseEntered(...)`
  - `void mouseExited(...)`
  - `String toString(...)`
  - `void canvasClick(...)`
  - `String getToolTipText(...)`
  - ... 和其他 10 个方法

### ComputeStyles

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/ComputeStyles.java`
- **内部依赖**: 8 个类
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.css.NormalOutput`
  - `org.fit.cssbox.css.Output`
  - `org.fit.cssbox.io.DOMSource`
  - ... 和其他 3 个
- **外部依赖**: `org.w3c`
- **主要公共方法** (1 个):
  - `void main(...)`

### ImageRenderer

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/ImageRenderer.java`
- **内部依赖**: 9 个类
  - `org.fit.cssbox.awt.GraphicsEngine`
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - ... 和其他 4 个
- **外部依赖**: `cz.vutbr`, `org.w3c`, `org.xml`
- **主要公共方法** (5 个):
  - `void setMediaType(...)`
  - `void setWindowSize(...)`
  - `void setLoadImages(...)`
  - `boolean renderURL(...)`
  - `void main(...)`

### SimpleBrowser

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/SimpleBrowser.java`
- **继承**: `javax.swing.JFrame`
- **内部依赖**: 8 个类
  - `org.fit.cssbox.awt.BrowserCanvas`
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - ... 和其他 3 个
- **外部依赖**: `org.w3c`
- **主要公共方法** (7 个):
  - `void mouseClicked(...)`
  - `void mousePressed(...)`
  - `void mouseReleased(...)`
  - `void mouseEntered(...)`
  - `void mouseExited(...)`
  - `void windowClosing(...)`
  - `void main(...)`

### StyleImport

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/StyleImport.java`
- **内部依赖**: 7 个类
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.css.NormalOutput`
  - `org.fit.cssbox.css.Output`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - ... 和其他 2 个
- **外部依赖**: `org.w3c`, `org.xml`
- **主要公共方法** (3 个):
  - `void dumpTo(...)`
  - `void dumpTo(...)`
  - `void main(...)`

### TextBoxes

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/demo/TextBoxes.java`
- **内部依赖**: 11 个类
  - `org.fit.cssbox.awt.GraphicsEngine`
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - ... 和其他 6 个
- **外部依赖**: `org.w3c`
- **主要公共方法** (1 个):
  - `void main(...)`

---

## org.fit.cssbox.io
**类数量**: 6

### ContentObserver

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/io/ContentObserver.java`
- **主要公共方法** (1 个):
  - `void contentLoadFailed(...)`

### DOMSource

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/io/DOMSource.java`
- **外部依赖**: `org.w3c`, `org.xml`
- **主要公共方法** (4 个):
  - `DocumentSource getDocumentSource(...)`
  - `String getCharset(...)`
  - `void setContentType(...)`
  - `Document parse(...)`

### DefaultDOMSource

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/io/DefaultDOMSource.java`
- **继承**: `DOMSource`
- **外部依赖**: `org.htmlunit`, `org.w3c`, `org.xml`
- **主要公共方法** (1 个):
  - `Document parse(...)`

### DefaultDocumentSource

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/io/DefaultDocumentSource.java`
- **继承**: `DocumentSource`
- **外部依赖**: `org.fit`
- **主要公共方法** (6 个):
  - `URL getURL(...)`
  - `InputStream getInputStream(...)`
  - `String getContentType(...)`
  - `String getUserAgent(...)`
  - `void setUserAgent(...)`
  - `void close(...)`

### DocumentSource

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/io/DocumentSource.java`
- **实现接口**: `java.io.Closeable`
- **主要公共方法** (4 个):
  - `URL getURL(...)`
  - `String getContentType(...)`
  - `InputStream getInputStream(...)`
  - `void close(...)`

### StreamDocumentSource

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/io/StreamDocumentSource.java`
- **继承**: `DocumentSource`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.io.DocumentSource`
- **主要公共方法** (4 个):
  - `URL getURL(...)`
  - `String getContentType(...)`
  - `InputStream getInputStream(...)`
  - `void close(...)`

---

## org.fit.cssbox.layout
**类数量**: 43

### BackgroundImage

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/layout/BackgroundImage.java`
- **外部依赖**: `cz.vutbr`, `org.slf4j`
- **主要公共方法** (10 个):
  - `ElementBox getOwner(...)`
  - `void setOwner(...)`
  - `Rectangle getComputedPosition(...)`
  - `Rectangle getComputedPosition(...)`
  - `boolean isRepeatX(...)`
  - `boolean isRepeatY(...)`
  - `float getImgX(...)`
  - `float getImgY(...)`
  - `float getImgWidth(...)`
  - `float getImgHeight(...)`

### BlockBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/BlockBox.java`
- **继承**: `ElementBox`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `void copyValues(...)`
  - `BlockBox copyBox(...)`
  - `void initBox(...)`
  - `void addSubBox(...)`
  - `void setStyle(...)`
  - `String toString(...)`
  - `boolean mayContainBlocks(...)`
  - `boolean containsBlocks(...)`
  - `void setFloats(...)`
  - `void setOwnerFloatList(...)`
  - ... 和其他 10 个方法

### BlockReplacedBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/BlockReplacedBox.java`
- **继承**: `BlockBox implements ReplacedBox`
- **外部依赖**: `org.w3c`
- **主要公共方法** (15 个):
  - `ReplacedContent getContentObj(...)`
  - `void setContentObj(...)`
  - `float getContentObjWidth(...)`
  - `float getContentObjHeight(...)`
  - `float getMaximalWidth(...)`
  - `float getMinimalWidth(...)`
  - `Rectangle getMinimalAbsoluteBounds(...)`
  - `boolean isWhitespace(...)`
  - `boolean isReplaced(...)`
  - `boolean marginsAdjoin(...)`
  - ... 和其他 5 个方法

### BlockTableBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/BlockTableBox.java`
- **继承**: `BlockBox`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (9 个):
  - `TableCaptionBox getCaption(...)`
  - `void setCaption(...)`
  - `TableBox getTable(...)`
  - `void setTable(...)`
  - `void initBox(...)`
  - `boolean canIncreaseWidth(...)`
  - `boolean doLayout(...)`
  - `float getMaximalWidth(...)`
  - `float getMinimalWidth(...)`

### Box

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/Box.java`
- **外部依赖**: `org.w3c`
- **主要公共方法** (20 个):
  - `void copyValues(...)`
  - `void initSubtree(...)`
  - `void adoptParent(...)`
  - `Node getNode(...)`
  - `int getOrder(...)`
  - `void setOrder(...)`
  - `int getSplitId(...)`
  - `VisualContext getVisualContext(...)`
  - `boolean isRootElement(...)`
  - `void makeRoot(...)`
  - ... 和其他 10 个方法

### BoxFactory

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/BoxFactory.java`
- **内部依赖**: 3 个类
  - `org.fit.cssbox.css.Counters`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (14 个):
  - `BrowserConfig getConfig(...)`
  - `void setConfig(...)`
  - `void setUseHTML(...)`
  - `boolean getUseHTML(...)`
  - `URL getBaseURL(...)`
  - `DOMAnalyzer getDecoder(...)`
  - `void reset(...)`
  - `Viewport createViewportTree(...)`
  - `void createBoxTree(...)`
  - `ElementBox createElementBox(...)`
  - ... 和其他 4 个方法

### BrowserConfig

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/BrowserConfig.java`
- **内部依赖**: 5 个类
  - `org.fit.cssbox.io.ContentObserver`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - `org.fit.cssbox.io.DefaultDocumentSource`
  - `org.fit.cssbox.io.DocumentSource`
- **外部依赖**: `cz.vutbr`, `org.slf4j`
- **主要公共方法** (20 个):
  - `Color getViewportBackgroundColor(...)`
  - `void setViewportBackgroundColor(...)`
  - `boolean getLoadImages(...)`
  - `void setLoadImages(...)`
  - `boolean getLoadBackgroundImages(...)`
  - `void setLoadFonts(...)`
  - `boolean isLoadFonts(...)`
  - `void setLoadBackgroundImages(...)`
  - `int getImageLoadTimeout(...)`
  - `void setImageLoadTimeout(...)`
  - ... 和其他 10 个方法

### CSSDecoder

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/CSSDecoder.java`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (8 个):
  - `VisualContext getContext(...)`
  - `void setContext(...)`
  - `float getLength(...)`
  - `float getLength(...)`
  - `double getAngle(...)`
  - `Rectangle computeReplacedObjectSize(...)`
  - `float applyWidthLimits(...)`
  - `float applyHeightLimits(...)`

### ContentImage

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/ContentImage.java`
- **主要公共方法** (1 个):
  - `URL getUrl(...)`

### Dimension

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/Dimension.java`
- **主要公共方法** (2 个):
  - `void setSize(...)`
  - `String toString(...)`

### ElementBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/ElementBox.java`
- **继承**: `Box`
- **内部依赖**: 2 个类
  - `org.fit.cssbox.css.BackgroundDecoder`
  - `org.fit.cssbox.css.CSSUnits`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `void copyValues(...)`
  - `ElementBox copyBox(...)`
  - `void initSubtree(...)`
  - `Element getElement(...)`
  - `boolean rendersBackground(...)`
  - `void setStyle(...)`
  - `NodeData getStyle(...)`
  - `String getStyleString(...)`
  - `String getPositionString(...)`
  - `String getStylePropertyValue(...)`
  - ... 和其他 10 个方法

### Engine

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/layout/Engine.java`
- **内部依赖**: 3 个类
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.css.FontTable`
  - `org.fit.cssbox.render.BoxRenderer`
- **外部依赖**: `org.slf4j`
- **主要公共方法** (17 个):
  - `BrowserConfig getConfig(...)`
  - `void setConfig(...)`
  - `ElementBox getRootBox(...)`
  - `Viewport getViewport(...)`
  - `void setViewport(...)`
  - `void setRootElement(...)`
  - `DOMAnalyzer getDecoder(...)`
  - `URL getBaseUrl(...)`
  - `void createLayout(...)`
  - `void createLayout(...)`
  - ... 和其他 7 个方法

### FloatList

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/FloatList.java`
- **主要公共方法** (10 个):
  - `BlockBox getOwner(...)`
  - `void add(...)`
  - `int size(...)`
  - `BlockBox getBox(...)`
  - `float getMaxY(...)`
  - `float getLastY(...)`
  - `float getWidth(...)`
  - `float getNextY(...)`
  - `float getMaxYForOwner(...)`
  - `float getNextY(...)`

### FontInfo

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/FontInfo.java`
- **主要公共方法** (8 个):
  - `String getFamily(...)`
  - `void setFamily(...)`
  - `float getSize(...)`
  - `void setSize(...)`
  - `boolean isBold(...)`
  - `void setBold(...)`
  - `boolean isItalic(...)`
  - `void setItalic(...)`

### HTMLBoxFactory

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/HTMLBoxFactory.java`
- **内部依赖**: 4 个类
  - `org.fit.cssbox.css.HTMLNorm`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - `org.fit.cssbox.io.DocumentSource`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`, `org.xml`
- **主要公共方法** (2 个):
  - `boolean isTagSupported(...)`
  - `ElementBox createBox(...)`

### ImageCache

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/ImageCache.java`

### ImageLoader

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/ImageLoader.java`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.render.BoxRenderer`
- **主要公共方法** (1 个):
  - `ContentImage loadImage(...)`

### Inline

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/Inline.java`
- **主要公共方法** (13 个):
  - `float getLineHeight(...)`
  - `float getMaxLineHeight(...)`
  - `float getBaselineOffset(...)`
  - `float getBelowBaseline(...)`
  - `float getTotalLineHeight(...)`
  - `float getHalfLead(...)`
  - `float getFirstLineLength(...)`
  - `float getLastLineLength(...)`
  - `boolean containsLineBreak(...)`
  - `boolean finishedByLineBreak(...)`
  - ... 和其他 3 个方法

### InlineBlockBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/InlineBlockBox.java`
- **继承**: `BlockBox implements InlineElement`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (20 个):
  - `void setStyle(...)`
  - `void setLineBox(...)`
  - `LineBox getLineBox(...)`
  - `float getLineboxOffset(...)`
  - `float getMaxLineHeight(...)`
  - `float getBaselineOffset(...)`
  - `float getBelowBaseline(...)`
  - `float getTotalLineHeight(...)`
  - `float getHalfLead(...)`
  - `float getFirstLineLength(...)`
  - ... 和其他 10 个方法

### InlineBlockReplacedBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/InlineBlockReplacedBox.java`
- **继承**: `InlineBlockBox implements ReplacedBox`
- **外部依赖**: `org.w3c`
- **主要公共方法** (20 个):
  - `ReplacedContent getContentObj(...)`
  - `void setContentObj(...)`
  - `float getContentObjWidth(...)`
  - `float getContentObjHeight(...)`
  - `float getMaximalWidth(...)`
  - `float getMinimalWidth(...)`
  - `Rectangle getMinimalAbsoluteBounds(...)`
  - `boolean isWhitespace(...)`
  - `boolean isReplaced(...)`
  - `boolean canSplitAfter(...)`
  - ... 和其他 10 个方法

### InlineBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/InlineBox.java`
- **继承**: `ElementBox implements InlineElement`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `void copyValues(...)`
  - `InlineBox copyBox(...)`
  - `String toString(...)`
  - `void setStyle(...)`
  - `void setLineBox(...)`
  - `LineBox getLineBox(...)`
  - `float getBaselineOffset(...)`
  - `float getBelowBaseline(...)`
  - `float getTotalLineHeight(...)`
  - `float getMaxLineHeight(...)`
  - ... 和其他 10 个方法

### InlineElement

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/InlineElement.java`
- **继承**: `Inline`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (3 个):
  - `void setLineBox(...)`
  - `LineBox getLineBox(...)`
  - `float getLineboxOffset(...)`

### InlineReplacedBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/InlineReplacedBox.java`
- **继承**: `InlineBox implements ReplacedBox`
- **外部依赖**: `org.w3c`
- **主要公共方法** (20 个):
  - `ReplacedContent getContentObj(...)`
  - `void setContentObj(...)`
  - `float getContentObjWidth(...)`
  - `float getContentObjHeight(...)`
  - `float getMaximalWidth(...)`
  - `float getMinimalWidth(...)`
  - `Rectangle getMinimalAbsoluteBounds(...)`
  - `boolean isWhitespace(...)`
  - `boolean isReplaced(...)`
  - `boolean canSplitAfter(...)`
  - ... 和其他 10 个方法

### LengthSet

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/LengthSet.java`
- **主要公共方法** (2 个):
  - `void copy(...)`
  - `String toString(...)`

### LineBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/LineBox.java`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (20 个):
  - `String toString(...)`
  - `ElementBox getParent(...)`
  - `int getEnd(...)`
  - `void setEnd(...)`
  - `int getStart(...)`
  - `void setY(...)`
  - `float getY(...)`
  - `float getAbsoluteY(...)`
  - `void setStart(...)`
  - `float getWidth(...)`
  - ... 和其他 10 个方法

### ListItemBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/ListItemBox.java`
- **继承**: `BlockBox`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (9 个):
  - `void initBox(...)`
  - `void setStyle(...)`
  - `void draw(...)`
  - `boolean hasVisibleBullet(...)`
  - `String getListStyleType(...)`
  - `ReplacedImage getMarkerImage(...)`
  - `int getItemNumber(...)`
  - `String getMarkerText(...)`
  - `String formatItemNumber(...)`

### Rectangle

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/Rectangle.java`
- **主要公共方法** (15 个):
  - `float getX(...)`
  - `void setX(...)`
  - `float getY(...)`
  - `void setY(...)`
  - `float getWidth(...)`
  - `void setWidth(...)`
  - `float getHeight(...)`
  - `void setHeight(...)`
  - `void setLocation(...)`
  - `void setSize(...)`
  - ... 和其他 5 个方法

### ReplacedBox

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/layout/ReplacedBox.java`
- **主要公共方法** (4 个):
  - `ReplacedContent getContentObj(...)`
  - `void setContentObj(...)`
  - `float getContentObjWidth(...)`
  - `float getContentObjHeight(...)`

### ReplacedContent

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/layout/ReplacedContent.java`
- **主要公共方法** (8 个):
  - `ElementBox getOwner(...)`
  - `void setOwner(...)`
  - `void loadSizeDefs(...)`
  - `void doLayout(...)`
  - `void absolutePositions(...)`
  - `float getIntrinsicWidth(...)`
  - `float getIntrinsicHeight(...)`
  - `float getIntrinsicRatio(...)`

### ReplacedImage

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/ReplacedImage.java`
- **继承**: `ReplacedContent`
- **外部依赖**: `org.fit`, `org.slf4j`
- **主要公共方法** (8 个):
  - `VisualContext getVisualContext(...)`
  - `URL getUrl(...)`
  - `ContentImage getImage(...)`
  - `void setImage(...)`
  - `float getIntrinsicWidth(...)`
  - `float getIntrinsicHeight(...)`
  - `float getIntrinsicRatio(...)`
  - `String toString(...)`

### ReplacedText

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/ReplacedText.java`
- **继承**: `ReplacedContent`
- **内部依赖**: 2 个类
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
- **外部依赖**: `org.slf4j`, `org.w3c`
- **主要公共方法** (6 个):
  - `Viewport getContentViewport(...)`
  - `float getIntrinsicWidth(...)`
  - `float getIntrinsicHeight(...)`
  - `float getIntrinsicRatio(...)`
  - `void doLayout(...)`
  - `void absolutePositions(...)`

### StackingContext

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/StackingContext.java`
- **继承**: `HashMap<Integer, Vector<ElementBox>>`
- **主要公共方法** (3 个):
  - `ElementBox getElementBox(...)`
  - `Vector<ElementBox> getElementsForZIndex(...)`
  - `void registerChildContext(...)`

### TableBodyBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableBodyBox.java`
- **继承**: `BlockBox`
- **外部依赖**: `org.w3c`
- **主要公共方法** (16 个):
  - `boolean rendersBackground(...)`
  - `void addRow(...)`
  - `int getRowCount(...)`
  - `int getColumnCount(...)`
  - `TableRowBox getRow(...)`
  - `void setSpacing(...)`
  - `TableBox getOwnerTable(...)`
  - `void setOwnerTable(...)`
  - `float getMaximalWidth(...)`
  - `float getMinimalWidth(...)`
  - ... 和其他 6 个方法

### TableBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableBox.java`
- **继承**: `BlockBox`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (5 个):
  - `int getColumnCount(...)`
  - `boolean hasFixedWidth(...)`
  - `void initBox(...)`
  - `boolean doLayout(...)`
  - `void determineColumnCount(...)`

### TableCaptionBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableCaptionBox.java`
- **继承**: `BlockBox`
- **外部依赖**: `cz.vutbr`, `org.w3c`

### TableCellBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableCellBox.java`
- **继承**: `BlockBox`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `int getColspan(...)`
  - `int getRowspan(...)`
  - `void setColspan(...)`
  - `void setRowspan(...)`
  - `int getRow(...)`
  - `int getColumn(...)`
  - `TableRowBox getOwnerRow(...)`
  - `void setOwnerRow(...)`
  - `TableColumn getOwnerColumn(...)`
  - `void setOwnerColumn(...)`
  - ... 和其他 10 个方法

### TableColumn

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableColumn.java`
- **继承**: `BlockBox`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.css.HTMLNorm`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (17 个):
  - `void copyValues(...)`
  - `TableColumn copyBox(...)`
  - `int getSpan(...)`
  - `String getSpecifiedWidth(...)`
  - `void setSpecifiedWidth(...)`
  - `void setColumnWidth(...)`
  - `float getMaximalWidth(...)`
  - `void setMaximalWidth(...)`
  - `float getMinimalWidth(...)`
  - `void setMinimalWidth(...)`
  - ... 和其他 7 个方法

### TableColumnGroup

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableColumnGroup.java`
- **继承**: `TableColumn`
- **外部依赖**: `org.w3c`
- **主要公共方法** (2 个):
  - `int getSpan(...)`
  - `TableColumn getColumn(...)`

### TableRowBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TableRowBox.java`
- **继承**: `BlockBox`
- **外部依赖**: `org.w3c`
- **主要公共方法** (12 个):
  - `boolean rendersBackground(...)`
  - `void addCell(...)`
  - `int getCellCount(...)`
  - `TableCellBox getCell(...)`
  - `void rewind(...)`
  - `TableCellBox next(...)`
  - `boolean hasNext(...)`
  - `TableBodyBox getOwnerBody(...)`
  - `void setOwnerBody(...)`
  - `void computeEfficientMargins(...)`
  - ... 和其他 2 个方法

### TextBox

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/TextBox.java`
- **继承**: `Box implements Inline`
- **外部依赖**: `cz.vutbr`, `org.w3c`
- **主要公共方法** (20 个):
  - `void copyValues(...)`
  - `TextBox copyTextBox(...)`
  - `String toString(...)`
  - `void initBox(...)`
  - `void setParent(...)`
  - `String getText(...)`
  - `boolean isDeclaredVisible(...)`
  - `boolean isDisplayed(...)`
  - `boolean isVisible(...)`
  - `void setWhiteSpace(...)`
  - ... 和其他 10 个方法

### UnlimitedImageCache

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/UnlimitedImageCache.java`
- **实现接口**: `ImageCache`
- **主要公共方法** (4 个):
  - `void put(...)`
  - `ContentImage get(...)`
  - `void putFailed(...)`
  - `boolean hasFailed(...)`

### Viewport

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/layout/Viewport.java`
- **继承**: `BlockBox`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.render.BoxRenderer`
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`
- **主要公共方法** (20 个):
  - `Rectangle getVisibleRect(...)`
  - `void setVisibleRect(...)`
  - `float getCanvasWidth(...)`
  - `float getCanvasHeight(...)`
  - `BrowserConfig getConfig(...)`
  - `void setConfig(...)`
  - `void initSubtree(...)`
  - `String toString(...)`
  - `BoxFactory getFactory(...)`
  - `float getMinimalWidthLimit(...)`
  - ... 和其他 10 个方法

### VisualContext

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/layout/VisualContext.java`
- **内部依赖**: 3 个类
  - `org.fit.cssbox.css.CSSUnits`
  - `org.fit.cssbox.css.FontSpec`
  - `org.fit.cssbox.css.FontTable`
- **外部依赖**: `cz.vutbr`, `org.slf4j`
- **主要公共方法** (20 个):
  - `void copyVisualContext(...)`
  - `VisualContext create(...)`
  - `VisualContext getParentContext(...)`
  - `void setParentContext(...)`
  - `BrowserConfig getConfig(...)`
  - `FontTable getFontTable(...)`
  - `Viewport getViewport(...)`
  - `void setViewport(...)`
  - `boolean isRootContext(...)`
  - `void makeRootContext(...)`
  - ... 和其他 10 个方法

---

## org.fit.cssbox.misc
**类数量**: 2

### Base64Coder

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/misc/Base64Coder.java`
- **主要公共方法** (2 个):
  - `String encodeString(...)`
  - `String decodeString(...)`

### Coords

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/misc/Coords.java`
- **主要公共方法** (1 个):
  - `boolean eq(...)`

---

## org.fit.cssbox.render
**类数量**: 10

### BackgroundImageGradient

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/BackgroundImageGradient.java`
- **继承**: `BackgroundImage`
- **内部依赖**: 2 个类
  - `org.fit.cssbox.layout.BackgroundImage`
  - `org.fit.cssbox.layout.ElementBox`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (8 个):
  - `Gradient getGradient(...)`
  - `void setGradient(...)`
  - `float getIntrinsicWidth(...)`
  - `float getIntrinsicHeight(...)`
  - `float getIntrinsicRatio(...)`
  - `boolean hasIntrinsicWidth(...)`
  - `boolean hasIntrinsicHeight(...)`
  - `boolean hasIntrinsicRatio(...)`

### BackgroundImageImage

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/BackgroundImageImage.java`
- **继承**: `BackgroundImage`
- **内部依赖**: 3 个类
  - `org.fit.cssbox.layout.BackgroundImage`
  - `org.fit.cssbox.layout.ContentImage`
  - `org.fit.cssbox.layout.ElementBox`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (9 个):
  - `URL getUrl(...)`
  - `ContentImage getImage(...)`
  - `void setImage(...)`
  - `float getIntrinsicWidth(...)`
  - `float getIntrinsicHeight(...)`
  - `float getIntrinsicRatio(...)`
  - `boolean hasIntrinsicWidth(...)`
  - `boolean hasIntrinsicHeight(...)`
  - `boolean hasIntrinsicRatio(...)`

### BackgroundRepeater

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/BackgroundRepeater.java`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.layout.Rectangle`
- **主要公共方法** (1 个):
  - `void repeatImage(...)`

### BoxRenderer

- **类型**: interface
- **文件**: `src/main/java/org/fit/cssbox/render/BoxRenderer.java`
- **继承**: `Closeable`
- **内部依赖**: 5 个类
  - `org.fit.cssbox.layout.ElementBox`
  - `org.fit.cssbox.layout.ListItemBox`
  - `org.fit.cssbox.layout.ReplacedBox`
  - `org.fit.cssbox.layout.TextBox`
  - `org.fit.cssbox.layout.Viewport`
- **主要公共方法** (8 个):
  - `void init(...)`
  - `void startElementContents(...)`
  - `void finishElementContents(...)`
  - `void renderElementBackground(...)`
  - `void renderMarker(...)`
  - `void renderTextContent(...)`
  - `void renderReplacedContent(...)`
  - `void close(...)`

### ElementBackground

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/ElementBackground.java`
- **内部依赖**: 5 个类
  - `org.fit.cssbox.layout.BackgroundImage`
  - `org.fit.cssbox.layout.Dimension`
  - `org.fit.cssbox.layout.ElementBox`
  - `org.fit.cssbox.layout.Rectangle`
  - `org.fit.cssbox.layout.Viewport`
- **主要公共方法** (5 个):
  - `ElementBox getOwner(...)`
  - `Rectangle getBounds(...)`
  - `Rectangle getClipped(...)`
  - `boolean isViewportOwner(...)`
  - `boolean isZeroSize(...)`

### Gradient

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/render/Gradient.java`
- **主要公共方法** (6 个):
  - `boolean isRepeating(...)`
  - `void setRepeating(...)`
  - `List<GradientStop> getStops(...)`
  - `void addStop(...)`
  - `Float getLastLengthPx(...)`
  - `void recomputeStops(...)`

### GradientStop

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/GradientStop.java`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (6 个):
  - `Color getColor(...)`
  - `Float getPercentage(...)`
  - `Float getPxLength(...)`
  - `void setColor(...)`
  - `void setPercentage(...)`
  - `void setPxLength(...)`

### LinearGradient

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/LinearGradient.java`
- **继承**: `Gradient`
- **主要公共方法** (10 个):
  - `float getLength(...)`
  - `float getWidth(...)`
  - `float getHeight(...)`
  - `float getX1(...)`
  - `float getY1(...)`
  - `float getX2(...)`
  - `float getY2(...)`
  - `float getEfficientX2(...)`
  - `float getEfficientY2(...)`
  - `void setAngleDeg(...)`

### RadialGradient

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/render/RadialGradient.java`
- **继承**: `Gradient`
- **内部依赖**: 1 个类
  - `org.fit.cssbox.layout.Rectangle`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (16 个):
  - `boolean isCircle(...)`
  - `float getCx(...)`
  - `float getCy(...)`
  - `float getRx(...)`
  - `float getRy(...)`
  - `float getEfficientRx(...)`
  - `float getEfficientRy(...)`
  - `float getLength(...)`
  - `Rectangle getBgRect(...)`
  - `float getGradWidth(...)`
  - ... 和其他 6 个方法

### StructuredRenderer

- **类型**: abstract class
- **文件**: `src/main/java/org/fit/cssbox/render/StructuredRenderer.java`
- **实现接口**: `BoxRenderer`
- **内部依赖**: 4 个类
  - `org.fit.cssbox.awt.BackgroundBitmap`
  - `org.fit.cssbox.awt.BitmapImage`
  - `org.fit.cssbox.css.BackgroundDecoder`
  - `org.fit.cssbox.layout.*`
- **外部依赖**: `cz.vutbr`
- **主要公共方法** (9 个):
  - `float getRootWidth(...)`
  - `float getRootHeight(...)`
  - `void init(...)`
  - `Viewport getViewport(...)`
  - `ElementBox getViewportBackgroundSource(...)`
  - `BackgroundDecoder getViewportBackground(...)`
  - `void renderElementBackground(...)`
  - `void renderReplacedContent(...)`
  - `void renderMarker(...)`

---

## org.fit.cssbox.testing
**类数量**: 4

### ImageComparator

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/testing/ImageComparator.java`
- **主要公共方法** (3 个):
  - `float getErrorRate(...)`
  - `String getErrorDescription(...)`
  - `BufferedImage getDifferenceImage(...)`

### ReferenceResults

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/testing/ReferenceResults.java`
- **继承**: `LinkedHashMap<String, Float>`
- **主要公共方法** (3 个):
  - `int getSuccessCnt(...)`
  - `int getFailCnt(...)`
  - `int getFatalCnt(...)`

### ReferenceTestCase

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/testing/ReferenceTestCase.java`
- **实现接口**: `Callable<Float>`
- **内部依赖**: 8 个类
  - `org.fit.cssbox.awt.GraphicsEngine`
  - `org.fit.cssbox.css.CSSNorm`
  - `org.fit.cssbox.css.DOMAnalyzer`
  - `org.fit.cssbox.io.DOMSource`
  - `org.fit.cssbox.io.DefaultDOMSource`
  - ... 和其他 3 个
- **外部依赖**: `cz.vutbr`, `org.slf4j`, `org.w3c`, `org.xml`
- **主要公共方法** (4 个):
  - `void setBatch(...)`
  - `String getName(...)`
  - `Float call(...)`
  - `float performTest(...)`

### TestBatch

- **类型**: class
- **文件**: `src/main/java/org/fit/cssbox/testing/TestBatch.java`
- **内部依赖**: 2 个类
  - `org.fit.cssbox.io.DefaultDOMSource`
  - `org.fit.cssbox.io.DefaultDocumentSource`
- **外部依赖**: `org.slf4j`, `org.w3c`, `org.xml`
- **主要公共方法** (10 个):
  - `int getTestCount(...)`
  - `void runTestsSingleList(...)`
  - `void runTestsSingleList(...)`
  - `void runTests(...)`
  - `void runTests(...)`
  - `void runTestsInSequence(...)`
  - `float runTest(...)`
  - `Map<String, Float> getResults(...)`
  - `float runTestByName(...)`
  - `void saveResults(...)`

---

