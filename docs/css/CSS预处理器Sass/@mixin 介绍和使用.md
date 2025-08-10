`@mixin` 是 CSS 预处理器 Sass（Syntactically Awesome Style Sheets）中的一个特性，用于创建可重用的样式块。通过定义 mixins，您可以将一组 CSS 规则封装在一起，以便在多个地方重复使用，减少代码冗余并提高可维护性。

### 主要作用

1. **代码重用**：通过定义 mixins，您可以在多个选择器中重复使用相同的 CSS 属性，而无需每次重新编写。

2. **参数化**：mixins 可以接受参数，从而使它们更灵活并且能够适应不同的场景。

3. **简化复杂样式**：将复杂的样式逻辑封装在 mixins 中，使主样式表更易读。

4. **动态生成 CSS**：您可以根据传入的参数生成不同的 CSS 规则，甚至可以根据条件动态实现不同的样式。

### 用法示例

#### 1. **基本示例**

```scss
@mixin rounded-corners {
  border-radius: 5px;
}

.button {
  @include rounded-corners; // 应用 mixin
  background-color: blue;
  color: white;
}
```

#### 2. **带参数的 mixin**

```scss
@mixin box-shadow($shadow-color, $x-offset, $y-offset, $blur-radius) {
  box-shadow: $x-offset $y-offset $blur-radius $shadow-color;
}

.card {
  @include box-shadow(#333, 0px, 2px, 5px); // 应用 mixin，传入参数
}
```

#### 3. **条件逻辑**

```scss
@mixin responsive-font-size($size) {
  font-size: $size;

  @media (max-width: 600px) {
    font-size: $size * 0.8; // 小屏幕缩小字体
  }
}

.heading {
  @include responsive-font-size(2rem);
}
```

#### 4. **多重应用**

可以将多个 mixins 组合在一起，提高样式的复用性。

```scss
@mixin flex-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

.container {
  @include flex-center;
  height: 100vh;
}
```

### 用途总览

1. **创建可重用组件**：定义按钮、卡片等常用 UI 元素样式。
2. **响应式设计**：根据不同屏幕尺寸自动调整样式。
3. **限于样式**：根据不同场景应用不同的样式，减少重复代码。
4. **简化复杂的 CSS**：将复杂的样式逻辑封装，使主样式表更简洁。

### 总结

`@mixin` 是 Sass 中一个非常强大的功能，通过其带来的代码重用、参数化和逻辑条件能力，可以显著提高 CSS 的可维护性和灵活性。适当地使用 mixins 可以让您的样式逻辑清晰，并使团队协作更加顺畅。