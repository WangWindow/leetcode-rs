# LeetCode Rust Solutions

本项目收录了我使用 Rust 语言实现的 LeetCode 算法题解，作为个人刷题记录与 Rust 练习仓库。

## 目录

- 题解代码按题号存放在 `src/problems/`，每道题在 `src/problems/p{题号}.rs` 中维护：
- 每题在文件头使用 rustdoc 注释保存题目元数据与 Markdown 题面
- 新题可通过 [`leetcode-creator`](https://github.com/WangWindow/leetcode-creator) 一键抓取题面和 Rust 模板并生成样板

## 如何使用

> [!NOTE]
>
> `leetcode-creator` 并不是本仓库的依赖，而是一个独立工具，安装和使用步骤如下：

1. 安装 `leetcode-creator` 到当前仓库根目录

2. 在本仓库根目录生成新题

```sh
./leetcode-creator add 1
```

3. 更新已有题目的头部元数据：

```sh
./leetcode-creator update 1
```

## 使用 `cargo test` 编写和运行样例

```sh
cargo test
```

## 贡献

欢迎提交 PR 或 issue，共同完善题解内容。

---

![Rustacean Logo](https://www.rustacean.net/assets/rustacean-orig-noshadow.png)
