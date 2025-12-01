# riscfetch

RISC-V専用のシステム情報表示ツール。fastfetchにインスパイアされたRISC-V特化の情報を表示します。

## インストール

```bash
cargo install riscfetch
```

または、ソースからビルド：

```bash
git clone https://github.com/kako-jun/riscfetch.git
cd riscfetch
cargo build --release
```

## 使い方

```bash
# 基本的な使用
riscfetch

# アニメーション付きスプラッシュ画面
riscfetch --splash

# ロゴスタイル変更
riscfetch --logo sifive

# ベンチマーク実行
riscfetch --benchmark

# 組み合わせ
riscfetch --splash --logo sifive --benchmark
```

## 表示される情報

- **Board**: ボードモデル（VisionFive 2, SiFive Unmatched等）
- **CPU**: RV64IMAFDC等のISA情報
- **Harts**: ハードウェアスレッド数
- **SoC**: System-on-Chip情報
- **ISA Extensions**: M, A, F, D, C, V, Z拡張の詳細
- **Vector**: ベクトル拡張の有無
- **Cache**: L1D, L1I, L2キャッシュ情報
- **Memory**: メモリ使用量
- **Kernel**: カーネルバージョン
- **OS**: ディストリビューション名
- **Uptime**: 稼働時間

## 対応ボード

- StarFive VisionFive 2
- SiFive HiFive Unmatched / Unleashed
- Milk-V Mars / Pioneer
- T-Head boards

## 出力例

```
      ____  ____  ____   ____      __  __
     / __ \/_  _\/ ___\ / ___|    / / / /
    / /_/ / / /  \___ \/ /   ____/ / / /
   / _, _/ / /  /___/ / /___/___/ /_/ /
  /_/ |_| /_/  /_____/\____/    \____/

        RISC-V Architecture Info

🖥️  Board: StarFive VisionFive 2
🧠 CPU: RV64IMAFDC
⚙️  Harts: 4 harts
🏗️  SoC: starfive,jh7110
🧪 ISA: M (Multiply), A (Atomic), F (Float), D (Double), C (Compressed)
📐 Vector: Enabled (V extension)
💾 Cache: L1D: 32K, L1I: 32K, L2: 2048K
🧮 Memory: 3.45 GiB / 8.00 GiB
🐧 Kernel: 6.5.0-riscv64
🕹️  OS: Debian GNU/Linux 12 (bookworm)
🚀 Uptime: 3h 42m
```

非RISC-Vシステムでは `Sorry, not RISC-V 😢` と表示されます。

## fastfetchとの併用

riscfetchはfastfetchと**競合せず補完**します：
- **fastfetch**: 汎用Linuxシステム情報
- **riscfetch**: RISC-V特有の情報

両方を使うことで、RISC-Vセットアップの全体像を表示できます。

## ライセンス

MIT
