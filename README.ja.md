# riscfetch

[![CI](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/riscfetch.svg)](https://crates.io/crates/riscfetch)
[![docs.rs](https://img.shields.io/docsrs/riscfetch-core)](https://docs.rs/riscfetch-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [中文](README.zh-CN.md) | [日本語](README.ja.md)

RISC-V システム情報ツール。ISA 拡張、hart 数、ハードウェア ID などを表示します。

**RISC-V 専用。** 他のアーキテクチャでは終了します。

![riscfetch screenshot](assets/screenshot.webp)

## 特徴

- **144 拡張に対応**（Z 拡張 98 種 + S 拡張 46 種）
- **カテゴリ別表示**で見やすい出力
- **13 種類のベンダーロゴ**（Pine64、WCH 含む）
- スクリプト向け JSON 出力
- 詳細説明モード (`-e`)

## なぜ RISC-V？

世界中のアニメ、映画、食べ物が好きです。クールな技術と未来が好きなだけ。RISC-V は触っていて楽しいので、neofetch 風のツールが欲しくなりました。

## インストール

### crates.io から

```bash
cargo install riscfetch
```

### GitHub Releases から

```bash
# 最新リリースをダウンロード
curl -LO https://github.com/kako-jun/riscfetch/releases/latest/download/riscfetch-linux-riscv64

# 実行権限を付与
chmod +x riscfetch-linux-riscv64

# PATH に移動
sudo mv riscfetch-linux-riscv64 /usr/local/bin/riscfetch
```

## 使い方

```bash
riscfetch              # 標準出力
riscfetch -a           # 全144拡張を ✓/✗ で表示
riscfetch -a -e        # 全拡張を説明付きで表示
riscfetch -r           # RISC-V 固有情報のみ（OS、メモリ等を除外）
riscfetch -e           # 各 ISA 拡張の説明を表示
riscfetch -j           # JSON 出力
riscfetch -a -j        # 全拡張を JSON 出力
riscfetch -s           # アニメーション付きスプラッシュ
riscfetch -b           # ベンチマーク実行
riscfetch -l pine64    # Pine64 ロゴを使用
```

## 出力例

拡張はカテゴリ別にグループ化されます：

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

## オプション

| フラグ | 説明 |
|--------|------|
| `-r, --riscv-only` | RISC-V 固有情報のみ表示（OS、メモリ、稼働時間を除外） |
| `-e, --explain` | 各拡張の意味を表示 |
| `-a, --all` | 全144拡張を ✓/✗ チェックマーク付きで表示 |
| `-j, --json` | 機械可読な JSON 出力 |
| `-s, --splash` | アニメーション付きスプラッシュ |
| `-b, --benchmark` | ISA 固有のベンチマーク |
| `-l, --logo <VENDOR>` | ベンダーロゴ（下記参照） |
| `--style <STYLE>` | ロゴスタイル: normal, small, none |

### 対応ベンダー（13 種類）

| ベンダー | 説明 |
|----------|------|
| `default` | 汎用 RISC-V ロゴ |
| `sifive` | SiFive (HiFive Unmatched, Unleashed) |
| `starfive` | StarFive (VisionFive 2) |
| `thead` | T-Head/Alibaba (XuanTie C906, C910) |
| `milkv` | Milk-V (Duo, Mars, Pioneer) |
| `sipeed` | Sipeed (Lichee, Maix シリーズ) |
| `pine64` | Pine64 (Star64, Oz64) |
| `kendryte` | Kendryte/Canaan (K210, K510) |
| `allwinner` | Allwinner (D1) |
| `espressif` | Espressif (ESP32-C3, C6) |
| `spacemit` | SpacemiT (K1, Orange Pi RV2) |
| `sophgo` | Sophgo (CV1800B, SG2000) |
| `wch` | WCH (CH32V003, CH32V103) |

## 対応拡張

### 標準拡張（11 種類）
I, E, M, A, F, D, Q, C, B, V, H

### Z 拡張（98 種類）
カテゴリ別: Base, Hints, Cache, Conditional, Bit Manipulation, Cryptography, Floating Point, Compressed, Atomics, Memory Model, Multiply, Vector, Vector Crypto

### S 拡張（46 種類）
カテゴリ別: Virtual Memory, Supervisor, Machine, Hypervisor, Debug, User

完全なリストは [SPEC.md](crates/riscfetch-core/SPEC.md) を参照。

## fastfetch との併用

riscfetch は RISC-V 固有の情報を表示します。完全なシステム情報には fastfetch と併用してください：

```bash
fastfetch && riscfetch -r
```

## コントリビューション

Issue や Pull Request を歓迎します！

- バグ報告
- 機能リクエスト
- 新しい RISC-V ボードのサポート
- ドキュメントの改善

### テスト協力募集

テスト用ハードウェアが限られています。以下の環境でテストできる方は、結果を報告してください（動作・非動作問わず）：

- **RV32E**（16 レジスタ組み込み向け）- 例: ESP32-C3, CH32V003
- **Vector 非対応 CPU** - 例: VisionFive 2, Allwinner D1
- **異なる VLEN 値** - VLEN=128, 512, 1024 など

「動きました」という報告だけでも価値があります！Issue で `/proc/cpuinfo` と riscfetch の出力を共有してください。

## ライセンス

MIT
