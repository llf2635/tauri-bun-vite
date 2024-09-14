export default {
  // 配置 scss 语法支持，参考 https://github.com/postcss/postcss-scss
  syntax: 'postcss-scss',
  plugins: {
    // 配置参考 https://tailwind.nodejs.cn/docs/using-with-preprocessors
    'postcss-import': {},
    tailwindcss: {},
    // 配置参考 https://www.npmjs.com/package/postcss-preset-env?activeTab=readme
    "postcss-preset-env": {
      /* 使用 Stage 3 特性 + CSS 嵌套规则 */
      stage: 3,
      features: {
        "nesting-rules": true,
      },
      // 自动添加浏览器前缀
      autoprefixer: {
        grid: true,
      },
    },
    // 参考 https://tailwind.nodejs.cn/docs/optimizing-for-production
    ...(process.env.NODE_ENV === 'production' ? { cssnano: {} } : {})
  },
}
