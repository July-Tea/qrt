# QRT

A simple command-line tool to generate QR codes in the terminal.

一个在终端中生成二维码的简单命令行工具。

## Installation / 安装

```bash
cargo install qrt
```

## Usage / 使用方法

Generate a QR code from text or URL:

从文本或URL生成二维码：

```bash
# Basic usage / 基本用法
qrt "Hello, World!"
qrt "https://example.com"

# With size options / 使用尺寸选项
qrt -s small "text"
qrt --size medium "text"
qrt --size large "text"
```

### Size Options / 尺寸选项

- `small` - Small QR code / 小尺寸二维码
- `medium` - Medium QR code (default) / 中等尺寸二维码（默认）
- `large` - Large QR code / 大尺寸二维码

## Examples / 示例

```bash
# Generate QR code for a URL / 为URL生成二维码
qrt "https://github.com"