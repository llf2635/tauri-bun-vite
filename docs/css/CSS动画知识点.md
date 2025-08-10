CSS 动画是用于创建动态效果的技术，主要有两种方式：`transition` 和 `animation`。这两者虽然都可以实现动画效果，但在使用上有一些明显的区别和特点。

### 基本知识点

#### 1. `transition`
- **定义**: `transition` 用于在状态变化（如 :hover、:focus、:active 等）时创建过渡效果。
- **触发**: 主要依赖于 DOM 元素的状态变化（如从一个样式到另一个样式的变化）。
- **语法**:
  ```css
  .element {
      transition: property duration timing-function delay;
  }
  ```
    - `property`: 要应用过渡的 CSS 属性（如 `opacity`、`transform` 等）。
    - `duration`: 动画完成所需的时间（如 `0.5s`）。
    - `timing-function`: 控制速度变化的函数（如 `ease`、`linear`、`ease-in` 等）。
    - `delay`: 动画开始前的延迟（如 `0s`）。

- **示例**:
  ```css
  .box {
      width: 100px;
      height: 100px;
      background-color: blue;
      transition: transform 0.5s ease;
  }

  .box:hover {
      transform: scale(1.5);
  }
  ```

#### 2. `animation`
- **定义**: `animation` 是一种更复杂的动画，允许在多个状态间平滑过渡。可以定义关键帧，以便实现不同阶段的样式。
- **触发**: 可以在页面加载时自动开始，也可以通过类的添加等方式触发。
- **语法**:
  ```css
  .element {
      animation: name duration timing-function delay iteration-count direction fill-mode;
  }
  
  @keyframes name {
      from { ... }
      to { ... }
  }
  ```
    - `name`: 动画的名称，需与 `@keyframes` 中定义的名称相同。
    - `duration`: 动画完成所需的时间。
    - `timing-function`: 动画速度的变化。
    - `delay`: 动画开始前的延迟。
    - `iteration-count`: 动画循环的次数（如 `infinite` 表示无限循环）。
    - `direction`: 控制动画的方向（如 `normal`、`reverse` 等）。
    - `fill-mode`: 控制动画完成后状态的表现（如 `forwards`、`backwards`）。

- **示例**:
  ```css
  @keyframes slide {
      from {
          transform: translateX(0);
      }
      to {
          transform: translateX(100px);
      }
  }

  .box {
      animation: slide 1s ease-in-out infinite;
  }
  ```

### `transition` 和 `animation` 的主要区别

| 特点               | `transition`                 | `animation`                    |
|--------------------|-----------------------------|--------------------------------|
| **定义**           | 通过状态变化来定义动画     | 通过关键帧来定义动画      |
| **触发方式**       | 依赖于状态的变化（如伪类）  | 可以通过类、属性等多种方式触发 |
| **复杂度**         | 更简单，适用于简单的效果   | 支持更复杂的多帧动画        |
| **关键帧**         | 不支持                       | 支持 `@keyframes` 定义关键帧 |
| **循环**           | 不支持                       | 可以设置循环次数            |

### 使用建议

- **使用 `transition`**: 当你需要创建简单的、基于状态变化的效果时，比如按钮悬停、菜单展开等。
- **使用 `animation`**: 当你需要创建更复杂的、定时的、连续的动画时，比如封面动画、加载指示器等。

了解这些知识点后，你可以根据具体需求来选择使用 `transition` 还是 `animation`，实现你想要的动画效果。


## animation: 2.5s cubic-bezier(.25, 1, .30, 1) wipe-out-left both;

在 CSS 中，`animation` 属性用于指定动画的各个参数，您提到的这行代码：

```css
animation: 2.5s cubic-bezier(.25, 1, .30, 1) wipe-out-left both;
```

包含了多个部分，每个部分都有其特定的含义。让我们逐个解析：

1. **`2.5s`**:
    - 这是动画的持续时间，表示动画将持续 2.5 秒。

2. **`cubic-bezier(.25, 1, .30, 1)`**:
    - 这是一个自定义的时间函数，用于控制动画的速度变化。
    - `cubic-bezier` 函数的四个参数指定了动画的起始、中间和结束速度。这个函数会影响动画在不同时间点的加速度，给出更细致的控制效果。

3. **`wipe-out-left`**:
    - 这是一个动画的名称，它对应于您在 CSS 中使用的 `@keyframes` 规则。
    - 你需要在样式表中定义名为 `wipe-out-left` 的关键帧动画，指定这个动画的具体效果（例如，元素如何从一个状态过渡到另一个状态）。

4. **`both`**:
    - 这是动画的填充模式（`animation-fill-mode`），用于控制动画播放完毕后，元素的样式如何表现。
    - `both` 意味着在动画开始之前和结束之后，元素都会使用动画的样式。具体来说，它可以满足以下两个效果：
        - 在动画开始之前，即使该元素处于动画状态，也会保留动画的起始样式。
        - 在动画结束后，元素会保持动画的最终样式。

### 示例

以下是一个完整的示例，展示如何使用上述的动画属性：

```css
@keyframes wipe-out-left {
    0% {
        opacity: 1;
        transform: translateX(0);
    }
    100% {
        opacity: 0;
        transform: translateX(-100%);
    }
}

.element {
    animation: 2.5s cubic-bezier(.25, 1, .30, 1) wipe-out-left both;
}
```

在这个例子中，`.element` 将会在 2.5 秒内执行一个名为 `wipe-out-left` 的动画，采用指定的自定义时间函数，并在动画完成后停留在最终状态。

## 动画填充模式（`animation-fill-mode`） 

`animation-fill-mode` 是一个 CSS 属性，用于控制动画在完成时的状态表现。它决定了当动画播放结束后，元素的样式应该保持为动画的初始状态、结束状态，还是不应用动画样式。

### 可能的值

`animation-fill-mode` 可以接受以下值：

1. **`none`**:
    - 默认值。动画完成后，元素将不会保持任何动画相关的样式，它会返回到未应用动画的状态。

2. **`forwards`**:
    - 动画结束后，元素会保持在动画的结束状态。这意味着动画完成后，元素将应用动画最后一帧的样式。

3. **`backwards`**:
    - 动画开始前，元素会保持在动画的开始状态。这通常用于设置初始样式，使元素在动画开始之前即应用动画的第一帧的样式。

4. **`both`**:
    - 同时应用 `forwards` 和 `backwards` 的效果。动画开始前保持在开始状态，结束后保持在结束状态。

### 使用示例

```css
@keyframes example {
    0% {
        opacity: 0;
        transform: translateY(-20px);
    }
    100% {
        opacity: 1;
        transform: translateY(0);
    }
}

.element {
    animation: example 1s ease-in-out forwards; /* 动画结束后保持最后的状态 */
}
```

在这个例子中，`.element` 将会在 1 秒内执行 `example` 动画并且在动画完成后保持最后的样式，即它会变为完全可见并位于原来的位置。

### 总结

- **`animation-fill-mode`** 让你能够决定动画完成后元素的样式状态。
- 选择适当的值可以提升用户体验，使动画的表现更加连贯和自然。根据设计需求灵活运用 `forwards`、`backwards`、`both` 等选项，可以实现更复杂和直观的动画效果。


## clip-path: inset(0 0 0 0);

在 CSS 中，`clip-path` 属性用于定义一个可见区域，仅显示这个区域的内容，而遮住其他部分。`inset()` 是 `clip-path` 的一种形状函数，它创建一个矩形区域，可以通过指定哪个方向（顶部、右侧、底部、左侧）应当被缩进来确定可见区域的大小和形状。

### `inset()` 函数的结构

`inset()` 函数的语法通常如下：

```css
clip-path: inset(top right bottom left);
```

- `top`：从元素的顶部到可见区域的距离。
- `right`：从元素的右侧到可见区域的距离。
- `bottom`：从元素的底部到可见区域的距离。
- `left`：从元素的左侧到可见区域的距离。

如果只提供一个值，它会同时应用到所有四个方向。如果提供两个值，第一个值会应用到顶部和底部，第二个值应用到左右。

### 你的例子

在你的动画定义中：

```css
@keyframes wipe-out-left {
  from {
    clip-path: inset(0 0 0 0); /* 完全可见 */
  }
  to {
    clip-path: inset(0 100% 0 0); /* 完全不可见，从左向右消失 */
  }
}
```

- `from { clip-path: inset(0 0 0 0); }`：表示完全可见，没有任何缩进。
- `to { clip-path: inset(0 100% 0 0); }`：表示从左侧开始遮盖，向右侧使用了 100% 的遮盖，这意味着元素的整个右侧是可见的，而左侧则被完全隐藏。

### 总结

- `inset()` 用于创建矩形可视范围，控制哪些部分是可见的。
- 在你的动画中，使用 `clip-path: inset()` 可以实现元素逐渐从左侧消失的效果。