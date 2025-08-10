## 原生 animation 动画知识点

CSS 动画的 `animation` 属性由多个组成元素，通常包括以下几个部分：

### 1. **animation-name**

- **作用**: 指定要应用的动画名称，这个名称对应于定义的 `@keyframes` 动画。
- **如**: `animation-name: fadeIn;`

### 2. **animation-duration**

- **作用**: 定义动画完成一个周期所需的时间。可以使用时间单位，如秒（`s`）或毫秒（`ms`）。
- **如**: `animation-duration: 2s;`

### 3. **animation-timing-function**

- **作用**: 定义动画的速度曲线，控制动画在不同时间点的变化速率。常用的值有：
    - `linear`：均匀速度。
    - `ease`：开始慢，结束慢，中间快。
    - `ease-in`：开始慢，结束快。
    - `ease-out`：开始快，结束慢。
    - `ease-in-out`：开始和结束都慢，中间快。
    - `cubic-bezier(n,n,n,n)`：自定义速度曲线。
- **如**: `animation-timing-function: ease-in-out;`

### 4. **animation-delay**

- **作用**: 定义动画开始之前的延迟时间。这使得在动画开始之前，可以设置等待时间。
- **如**: `animation-delay: 0.5s;`

### 5. **animation-iteration-count**

- **作用**: 指定动画的重复次数，可以是一个数字或单词（如 `infinite`，表示无限循环）。
- **如**: `animation-iteration-count: infinite;`

### 6. **animation-direction**

- **作用**: 定义动画是否在播放完成后反向播放，可能的值包括：
    - `normal`（默认）：从头到尾播放。
    - `reverse`：从尾到头播放。
    - `alternate`：每次播放时改变方向。
    - `alternate-reverse`：每次播放时改变方向，从尾到头到头播放。
- **如**: `animation-direction: alternate;`

### 7. **animation-fill-mode**

- **作用**: 定义动画在播放前、播放后如何保留样式，可以取值：
    - `none`（默认）：动画结束后回到初始状态。
    - `forwards`：保留最后一帧的样式。
    - `backwards`：保留初始帧的样式。
    - `both`：同时适用 `forwards` 和 `backwards`。
- **如**: `animation-fill-mode: forwards;`

### 8. **animation-play-state**

- **作用**: 控制动画的播放状态，可以设置为 `running`（运行）或 `paused`（暂停）。
- **如**: `animation-play-state: paused;`

### 组合顺序

虽然这几个属性可以独立设置，但是在 CSS 中使用 `animation` 的语法时，通常会结合多个属性，并且可以按顺序列出。**顺序是有一定灵活性的**，但通常推荐的顺序是：

```css
animation: animation-name animation-duration animation-timing-function animation-delay animation-iteration-count animation-direction animation-fill-mode animation-play-state;
```

例如:

```css
.element {
    animation: fadeIn 2s ease-in-out 0s 1 normal forwards;
}
```

### 总结

CSS 动画的各组成元素可以单独设定，也可以使用 `animation` 属性来同时指定。虽然它们的顺序并不完全固定，但推荐按照一定的顺序来书写，以提升可读性和维护性。


## `@keyframes` 动画关键帧

`@keyframes` 是一个 CSS 规则，用于创建动画的关键帧。通过定义关键帧，您可以设置动画在不同时间点的样式状态。`@keyframes` 的常用语法和内容主要包括以下几个方面：

### 1. **基本语法**

```css
@keyframes animation-name {
  from {
    /* 起始样式 */
  }
  to {
    /* 结束样式 */
  }
}
```

或者使用百分比来定义更复杂的帧：

```css
@keyframes animation-name {
  0% {
    /* 起始样式 */
  }
  50% {
    /* 中间样式 */
  }
  100% {
    /* 结束样式 */
  }
}
```

### 2. **常用的关键字**

- **`from`** / **`to`**: 表示动画的起始和结束点，相当于 `0%` 和 `100%`。
- **`0%`**: 动画的开始。
- **`50%`**: 动画的中间状态。
- **`100%`**: 动画的结束。
- ****<key>**%**：定义动画在该百分比位置的样式。

### 3. **常用的动画属性**

在 `@keyframes` 中，通常会用到以下 CSS 属性：

- **`transform`**: 对元素进行旋转、缩放、平移等变换。
- **`opacity`**: 控制元素的透明度。
- **`color`**: 文本或背景颜色变化。
- **`background-color`**: 背景色变化。
- **`width`**, **`height`**: 元素尺寸变化。
- **`margin`**, **`padding`**: 外边距或内边距变化。

### 4. **例子**

下面是一个简单的例子，展示一个元素从透明到不透明的渐变效果：

```css
@keyframes fadeIn {
  from {
    opacity: 0; /* 起始透明度 */
  }
  to {
    opacity: 1; /* 结束透明度 */
  }
}

.element {
  animation: fadeIn 2s ease-in-out; /* 应用动画 */
}
```

### 5. **组合多种效果**

你也可以在同一个 `@keyframes` 中结合多个效果：

```css
@keyframes bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translateY(0); /* 初始位置 */
  }
  40% {
    transform: translateY(-30px); /* 弹起 */
  }
  60% {
    transform: translateY(-15px); /* 弹起的高度 */
  }
}

.element {
  animation: bounce 2s infinite; /* 无限弹跳 */
}
```

### 6. **使用多个动画**

你可以将多个 `@keyframes` 动画同时应用在同一元素上，只需用逗号分隔它们：

```css
.element {
  animation: fadeIn 2s ease-in-out, bounce 1s ease-in-out infinite;
}
```

### 总结

`@keyframes` 可以用来创建丰富的动画效果，通过定义不同的关键帧，可以精确控制动画在不同时间点的样式变化。使用合理的百分比和 CSS 属性，您可以实现各种动态效果来提升用户体验。


## `@keyframes` 动画的示例

要在 `@keyframes` 动画中实现元素尺寸的渐变效果，可以通过改变 `width`、`height` 或 `transform: scale()` 属性来实现。以下是几种实现元素尺寸渐变效果的常见方法：

### 方法 1: 使用 `transform: scale()`

使用 `transform: scale()` 是处理元素尺寸变换的推荐方式，因为它不会影响文档的流动性，同时也能保持元素的中心。

```html
<div class="box"></div>
```

```css
@keyframes scaleAnimation {
  0% {
    transform: scale(1); /* 原始大小 */
  }
  50% {
    transform: scale(1.5); /* 增大到 1.5 倍 */
  }
  100% {
    transform: scale(1); /* 返回到原始大小 */
  }
}

.box {
  width: 100px;
  height: 100px;
  background-color: blue;
  animation: scaleAnimation 2s ease-in-out infinite; /* 应用动画 */
}
```

### 方法 2: 直接改变 `width` 和 `height`

如果你希望直接设置元素的实际宽度和高度，可以通过改变 `width` 和 `height` 属性来实现：

```html
<div class="box"></div>
```

```css
@keyframes sizeAnimation {
  0% {
    width: 100px;
    height: 100px; /* 初始尺寸 */
  }
  50% {
    width: 200px;  /* 增加宽度 */
    height: 200px; /* 增加高度 */
  }
  100% {
    width: 100px;
    height: 100px; /* 返回到初始尺寸 */
  }
}

.box {
  background-color: red;
  animation: sizeAnimation 2s ease-in-out infinite; /* 应用动画 */
}
```

### 方法 3: 结合 `overflow: hidden`

如果你的元素可能会溢出其容器，可以通过设置 `overflow: hidden` 来避免显示出动画过程中可能出现的内容溢出：

```html
<div class="container">
  <div class="box"></div>
</div>
```

```css
.container {
  width: 150px;
  height: 150px;
  overflow: hidden; /* 隐藏溢出的内容 */
  border: 1px solid #000;
}

@keyframes sizeAnimation {
  0% {
    width: 100px;
    height: 100px;
  }
  50% {
    width: 200px;
    height: 200px;
  }
  100% {
    width: 100px;
    height: 100px;
  }
}

.box {
  background-color: green;
  animation: sizeAnimation 2s ease-in-out infinite; /* 应用动画 */
}
```

### 总结

无论是使用 `transform: scale()` 还是直接调整 `width` 和 `height`，都可以实现元素尺寸的渐变效果。使用 `scale()` 是更推荐的方法，因为它不会导致页面重新布局，性能也更好。根据具体需求选择合适的方法即可。