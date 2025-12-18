# riscfetch

[![CI](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/riscfetch.svg)](https://crates.io/crates/riscfetch)
[![docs.rs](https://img.shields.io/docsrs/riscfetch-core)](https://docs.rs/riscfetch-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [中文](README.zh-CN.md) | [日本語](README.ja.md)

RISC-V 系统信息工具。显示 ISA 扩展、hart 数量、硬件 ID 等信息。

**仅支持 RISC-V。** 在其他架构上会退出。

![riscfetch screenshot](assets/screenshot.webp)

## 特性

- **支持 144 种扩展**（98 种 Z 扩展 + 46 种 S 扩展）
- **分类显示**便于阅读
- **13 种厂商 logo**（包括 Pine64 和 WCH）
- 脚本友好的 JSON 输出
- 详细解释模式 (`-e`)

## 为什么选择 RISC-V？

我喜欢来自世界各地的动漫、电影和美食。只是一个喜欢酷技术和未来的爱好者。RISC-V 很有趣，我想为它做一个类似 neofetch 的工具。

## 安装

### 从 crates.io

```bash
cargo install riscfetch
```

### 从 GitHub Releases

```bash
# 下载最新版本
curl -LO https://github.com/kako-jun/riscfetch/releases/latest/download/riscfetch-linux-riscv64

# 添加执行权限
chmod +x riscfetch-linux-riscv64

# 移动到 PATH
sudo mv riscfetch-linux-riscv64 /usr/local/bin/riscfetch
```

## 使用方法

```bash
riscfetch              # 标准输出
riscfetch -a           # 显示全部144个扩展，带 ✓/✗ 标记
riscfetch -a -e        # 全部扩展带说明
riscfetch -r           # 仅显示 RISC-V 特定信息（排除 OS、内存等）
riscfetch -e           # 解释每个 ISA 扩展
riscfetch -j           # JSON 输出
riscfetch -a -j        # 全部扩展 JSON 输出
riscfetch -s           # 动画启动画面
riscfetch -b           # 运行基准测试
riscfetch -l pine64    # 使用 Pine64 logo
```

## 输出示例

扩展按类别分组显示：

```
ISA:        rv64imafdcv_zicsr_zifencei_zba_zbb_zbs_sstc...
Ext:        I M A F D C V
Z-Base:     Zicsr Zifencei Zicntr Zihpm
Z-Bit:      Zba Zbb Zbc Zbs
Z-Vector:   Zvl128b Zvl256b
S-Sup:      Sstc
Vector:     Enabled, VLEN>=256
Harts:      4 harts
HW IDs:     vendor:0x489 arch:0x8000000000000007 impl:0x0
Cache:      L1D:32K L1I:32K L2:2048K

--------------------------------

Board:      StarFive VisionFive 2
OS:         Ubuntu 24.04 LTS
Kernel:     6.8.0-riscv64
Memory:     3.45 GiB / 8.00 GiB
Uptime:     3h 42m
User:       user@visionfive2
```

## 选项

| 参数 | 说明 |
|------|------|
| `-r, --riscv-only` | 仅显示 RISC-V 特定信息（排除 OS、内存、运行时间） |
| `-e, --explain` | 显示每个扩展的含义 |
| `-a, --all` | 显示全部 144 种扩展，带 ✓/✗ 标记 |
| `-j, --json` | 机器可读的 JSON 输出 |
| `-s, --splash` | 动画启动画面 |
| `-b, --benchmark` | ISA 特定基准测试 |
| `-l, --logo <VENDOR>` | 厂商 logo（见下文） |
| `--style <STYLE>` | Logo 样式：normal、small、none |

### 支持的厂商（13 种）

| 厂商 | 说明 |
|------|------|
| `default` | 通用 RISC-V logo |
| `sifive` | SiFive (HiFive Unmatched, Unleashed) |
| `starfive` | StarFive 赛昉 (VisionFive 2) |
| `thead` | T-Head 平头哥/阿里巴巴 (玄铁 C906, C910) |
| `milkv` | Milk-V (Duo, Mars, Pioneer) |
| `sipeed` | Sipeed 矽速 (Lichee, Maix 系列) |
| `pine64` | Pine64 (Star64, Oz64) |
| `kendryte` | Kendryte 嘉楠 (K210, K510) |
| `allwinner` | Allwinner 全志 (D1) |
| `espressif` | Espressif 乐鑫 (ESP32-C3, C6) |
| `spacemit` | SpacemiT 进迭时空 (K1, Orange Pi RV2) |
| `sophgo` | Sophgo 算能 (CV1800B, SG2000) |
| `wch` | WCH 沁恒 (CH32V003, CH32V103) |

## 支持的扩展

### 标准扩展（11 种）
I, E, M, A, F, D, Q, C, B, V, H

### Z 扩展（98 种）
按类别分组：Base、Hints、Cache、Conditional、Bit Manipulation、Cryptography、Floating Point、Compressed、Atomics、Memory Model、Multiply、Vector、Vector Crypto

### S 扩展（46 种）
按类别分组：Virtual Memory、Supervisor、Machine、Hypervisor、Debug、User

完整列表请参见 [SPEC.md](crates/riscfetch-core/SPEC.md)。

## 配合 fastfetch 使用

riscfetch 显示 RISC-V 特定信息。配合 fastfetch 使用可获取完整系统信息：

```bash
fastfetch && riscfetch -r
```

## 贡献

欢迎提交 Issue 和 Pull Request！

- Bug 报告
- 功能请求
- 新 RISC-V 开发板支持
- 文档改进

### 需要测试帮助

我们的测试硬件有限。如果您能在以下环境测试，请报告结果（无论成功与否）：

- **RV32E**（16 寄存器嵌入式）- 如 ESP32-C3、CH32V003
- **无 Vector 扩展的 CPU** - 如 VisionFive 2、Allwinner D1
- **不同 VLEN 值** - VLEN=128、512、1024 等

即使是"正常工作"的报告也很有价值！请提交 Issue 并附上您的 `/proc/cpuinfo` 和 riscfetch 输出。

## 许可证

MIT
