# QRT

A simple command-line tool to generate and decode QR codes in terminal with PNG support.

一个在终端中生成和解码二维码的简单命令行工具。

## Installation / 安装

```bash
cargo install qrt
```

## Usage / 使用方法

### Generate QR Codes / 生成二维码

```bash
# Basic usage / 基本用法
qrt "Hello, World!"
qrt "https://example.com"

# With size options / 使用尺寸选项
qrt -s small "text"
qrt --size medium "text"
qrt --size large "text"

# Save as PNG image / 保存为PNG图片
qrt "Hello" --save           # saves as output.png / 保存为 output.png
qrt "Hello" --save my-qr     # saves as my-qr.png / 保存为 my-qr.png
```

### Decode QR Codes / 解码二维码

```bash
# Decode QR code from image / 从图片解码二维码
qrt --decode qrcode.png
qrt -d image.jpg
```

### Size Options / 尺寸选项

- `small` - Small QR code / 小尺寸二维码
- `medium` - Medium QR code (default) / 中等尺寸二维码（默认）
- `large` - Large QR code / 大尺寸二维码

### Save Options / 保存选项

- `--save` - Save as `output.png` in current directory / 在当前目录保存为 `output.png`
- `--save filename` - Save as `filename.png` in current directory / 在当前目录保存为 `filename.png`

## Examples / 示例

```bash
# Generate QR code for a URL / 为URL生成二维码
qrt "https://github.com"

# Save QR code as PNG image / 保存为PNG图片
qrt "Hello World" --save

# Save with custom filename / 自定义文件名保存
qrt "My QR Code" --save my-qr-code

# Large QR code saved / 大尺寸二维码保存
qrt "Large QR" --size large --save

# Decode existing QR code / 解码现有二维码
qrt --decode qrcode.png
```