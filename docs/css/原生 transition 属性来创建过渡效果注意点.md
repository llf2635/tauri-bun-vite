## 原生 transition 属性来创建过渡效果注意点

CSS 的 `transition` 属性用于实现属性值的平滑过渡，其组成元素包括以下几个部分：

### 1. **transition-property**

- **作用**: 指定要应用过渡效果的 CSS 属性，可以是单个属性、多个属性（用逗号分隔），或者也可以使用 `all` 来表示所有可过渡的属性。
- **如**: `transition-property: opacity;`

### 2. **transition-duration**

- **作用**: 定义过渡完成所需的时间，单位可以是秒（`s`）或毫秒（`ms`）。
- **如**: `transition-duration: 0.5s;`

### 3. **transition-timing-function**

- **作用**: 定义过渡的速度曲线，控制过渡在时间轴上的变化速率，可以取值如下：
    - `linear`：匀速过渡。
    - `ease`：缓慢开始和结束，中间加速。
    - `ease-in`：先慢后快。
    - `ease-out`：先快后慢。
    - `ease-in-out`：先慢后快，再慢。
    - `cubic-bezier(n,n,n,n)`：自定义速度曲线。
- **如**: `transition-timing-function: ease-in-out;`

### 4. **transition-delay**

- **作用**: 定义过渡开始之前的等待时间，可以设置为 `0s`，或其他延迟的时间。
- **如**: `transition-delay: 0.2s;`

### 组合顺序

虽然这些属性可以单独设置，但在 CSS 中使用 `transition` 时，可以组合在一起，并且可以按以下顺序列出：

```css
transition: transition-property transition-duration transition-timing-function transition-delay;
```

### 示例

```css
.element {
    transition: opacity 0.5s ease-in-out 0s; /* 当 opacity 属性变化时，应用过渡效果 */
}
```

### 总结

`transition` 的各个组成部分可以单独设置，也可以通过`transition` 属性同时列出。虽然顺序不是严格固定的，但为了提高可读性和维护性，通常建议按照上述顺序来书写。这使得其他开发者能够更容易理解过渡效果的配置。



## 不是所有 CSS 属性都可以应用 transition

CSS 的原生 `transition` 属性可以用来创建过渡效果，但并不是所有的 CSS 属性都能够应用 `transition`。以下是一些不能直接应用 `transition` 的情况：

### 1. **无法过渡的 CSS 属性**

某些 CSS 属性不支持过渡效果：

- **`display`**: 例如，当你改变元素的 `display` 属性为 `none`、`block` 或其他值时，无法过渡。
- **`position`**: 属性如 `static`、`relative`、`absolute` 和 `fixed` 不支持过渡。
- **`float`**: 浮动状态不能通过过渡动画来实施。
- **`overflow`**: 这个属性不能被动画。
- **`visibility`**: 虽然可以变化（`visible`、`hidden`），但没有过渡效果。
- **`z-index`**: 不能直接过渡。

### 2. **不支持动画的值**

有些值的改变也无法进行过渡，尤其是涉及到复杂计算的值：

- **复杂背景**：例如改变背景图像的 URL。
- **某些颜色**：虽然大多数颜色变化是可过渡的，但有些特定效果或属性（如 `background-image`）不支持。

### 3. **非视觉属性**

一些 CSS 属性虽然可以改动，但并不影响视觉表现，因此也无法进行过渡：

- **`content`**: 虽然可以通过 `::before` 和 `::after` 伪元素实现在页面中的变化，但 `content` 本身没有过渡效果。

### 4. **由于硬件加速限制的属性**

虽然大多数 CSS 属性可以应用 `transition`，但在某些情况下，使用硬件加速的属性可能会制约过渡效果：

- **某些变换**：像 `transform` 和 `opacity` 通常支持过渡，但如果设置不当（例如，设置为 `none`），在某些情况下也可能阻止动画效果。

### 5. **Transition 上的限制**

- **简写属性**：如果使用简写形式（如 `margin`、`padding` 等），过渡可能不会按预期工作。你需要明确指定每一个子项目（例如，`margin-top`、`margin-bottom`）。

### 结论

如果你希望使用 CSS 过渡效果，确保你利用的是可动画化的属性。通常情况下，`opacity`、`transform`、`background-color` 和大多数边距和填充属性是可以安全地进行过渡的。选择合适的属性可以为用户体验带来便利，并提升页面的动态效果。




## 如何解决 transition 不能应用在某些属性上？

对于无法直接使用 CSS `transition` 的属性（例如 `display`），你可以通过一些间接的方法来实现过渡动画。以下是几种常见的实现方式：

### 1. **利用 `opacity` 和 `visibility`**

通过结合使用 `opacity` 和 `visibility` 来实现渐变效果，同时在 JavaScript 中控制 `display` 属性。

```html
<template>
  <div>
    <button @click="isVisible = !isVisible">Toggle</button>
    <div :class="{ 'fade': isVisible }" @transitionend="handleTransitionEnd">
      This is a content
    </div>
  </div>
</template>

<style>
.fade {
  opacity: 0;
  visibility: hidden; /* 初始状态为不可见 */
  transition: opacity 0.5s ease;
}

.fade-visible {
  opacity: 1;
  visibility: visible; /* 过渡到可见状态 */
}
</style>

<script>
export default {
  data() {
    return {
      isVisible: false,
    };
  },
  methods: {
    handleTransitionEnd() {
      if (!this.isVisible) {
        // 过渡结束时，设置 display 为 none
        this.$refs.content.style.display = 'none';
      }
    },
  },
  watch: {
    isVisible(newValue) {
      if (newValue) {
        this.$refs.content.style.display = 'block';
      } else {
        this.$refs.content.style.display = 'block';
      }
    },
  },
};
</script>
```

### 2. **使用 `max-height` 来模拟展开和收起**

你可以使用 `max-height` 来模拟一个元素的展开和收起效果。这种方式适用于一些具有固定高度的元素。

```html
<template>
  <div>
    <button @click="isVisible = !isVisible">Toggle</button>
    <div :style="{ maxHeight: isVisible ? '100px' : '0px' }" class="collapse">
      This is a content
    </div>
  </div>
</template>

<style>
.collapse {
  overflow: hidden;
  transition: max-height 0.5s ease;
}
</style>

<script>
export default {
  data() {
    return {
      isVisible: false,
    };
  },
};
</script>
```

### 3. **使用 JavaScript 动画库**

如果需要更复杂的动画效果，可以考虑使用 JavaScript 动画库，比如 GSAP、Anime.js 之类的。这些库提供了更强大的动画功能。

### 4. **CSS 动画 (`@keyframes`)**

尽管 CSS 动画与过渡效果略有不同，但你也可以使用 CSS 动画来控制元素的进入和离开。

```html
<template>
  <div>
    <button @click="isVisible = !isVisible">Toggle</button>
    <div v-if="isVisible" class="fade-in-out">This is a content</div>
  </div>
</template>

<style>
.fade-in-out {
  animation: fade 0.5s forwards;
}

@keyframes fade {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>

<script>
export default {
  data() {
    return {
      isVisible: false,
    };
  },
};
</script>
```

### 5. **使用 Vue 内置过渡动画组件 (`<transition>`)**

<template>
  <transition name="fade">
    <div v-if="isVisible" class="box">可见内容</div>
  </transition>
  <button @click="isVisible = !isVisible">切换可见性</button>
</template>

<style>
.fade-enter-active, .fade-leave-active {
    transition: opacity 0.5s;
}
.fade-enter, .fade-leave-to /* .fade-leave-active in <2.1.8 */ {
    opacity: 0;
}
</style>

### 总结

对于无法直接过渡的 CSS 属性，通常需要通过结合使用 `opacity`、`visibility`、`max-height`，或者使用 JavaScript 动画库等方法来实现过渡效果。这些技巧能够帮助你克服 `transition` 的限制，达到预期的动画效果。



## v-show 和 v-if 的区别

v-show 和 CSS transition

    v-show: 当你使用 v-show 时，VueJS 是通过改变元素的 display 属性来控制元素的显示与隐藏。在这个过程中，transition 无法对此生效，因为 display: none 直接阻止了元素的渲染，从而不允许 CSS 动画或过渡效果应用。

v-if 和 CSS transition

    v-if: 当你使用 v-if 时，元素在 DOM 中完全被添加或移除，因此任意 transition 都可以通过 Vue 的过渡系统来处理，通常可以直接使用 CSS 的 transition 属性来实现进入和离开的动画效果。
