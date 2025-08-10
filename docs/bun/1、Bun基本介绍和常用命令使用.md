# Bun 简介

## 什么是 Bun？

Bun 是一个一体化的 JavaScript 和 TypeScript 应用运行时环境，旨在提供快速的性能和简化的开发体验。 它被设计为一个轻量级但功能强大的平台，用于执行 JavaScript 代码。 作为 Node.js 的替代品，Bun 是一个全功能的 JavaScript 运行时、打包器和测试运行器，以单一可执行文件 bun 的形式提供。

## Bun 的作用

Bun 的主要作用是构建、运行和测试 JavaScript/TypeScript 应用程序，使用一个快速的工具即可完成所有操作。 它提供了闪电般的性能，简化了现代 Web 开发流程。 Bun 内置支持浏览器中可用的 Web 标准 API，如 fetch、Request、Response、URL、blob、WebSocket 和 JSON 等，使开发者能够使用熟悉的 Web API 进行开发。

## Bun 常用命令及作用

| 命令 | 作用 | 对应传统工具命令 |
|------|------|----------------|
| `bun run` | 执行 JavaScript/TypeScript 文件或运行项目脚本 | `npm run` / `yarn run`  |
| `bun init` 或 `bun create` | 初始化新项目，创建基本项目结构 | `npm init` / `yarn init`  |
| `bun install` | 安装项目依赖（作为内置的包管理器） | `npm install` / `yarn install` |
| `bun test` | 执行测试脚本 | `npm test` / `yarn test` |
| `bun build` | 打包项目代码 | 无直接对应，类似于 webpack 或 vite 的构建功能 |
| `Bun.$` | 在 Bun 中运行 shell 命令，支持类似 bash 的语法  | 类似于 shell 脚本中的命令执行 |
| `bun x` | 运行全局安装的包 | `npx` |

Bun 的 CLI 设计简洁高效，可以在 Windows 上使用 bun shell 支持类似 bash 的语法和许多常用命令，提高了跨平台开发的便利性。 作为一个全能型工具，Bun 提供了从项目初始化、依赖管理、代码执行到测试和打包的一站式解决方案，大大简化了 JavaScript/TypeScript 开发工作流。