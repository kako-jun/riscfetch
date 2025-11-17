# riscfetch - RISC-V Specialized System Information Tool

## 🎯 Vision

**世界初のRISC-V専用fetchツール** - RISC-Vアーキテクチャに特化した情報表示ツールを提供し、RISC-Vコミュニティに貢献する。

## 💡 Project Concept

### Core Philosophy

fastfetchやneofetchなどの汎用fetchツールとは**競合せず補完する**存在として位置づける：

- **汎用情報**: fastfetch/neofetchに任せる (Shell, Terminal, Packages, DE/WM, etc.)
- **RISC-V特化情報**: riscfetchで提供 (Hart count, ISA extensions, Board detection, Vector info, etc.)

### Why riscfetch?

1. **Need**: 既存のfetchツールはRISC-V特有の情報（Hart、ISA拡張、ボード名等）を表示できない
2. **Community**: RISC-Vユーザーは少数派 → 自分のセットアップを自慢したい
3. **Education**: ISA拡張の意味を学べる教育的価値
4. **First Mover**: 調査の結果、RISC-V専用fetchツールは世界に存在しない

## 🎨 Design Principles

### 1. RISC-V Exclusive
- RISC-V以外では "Sorry, not RISC-V 😢" と表示して終了
- RISC-Vでしか動かないことが特徴

### 2. Beautiful & Modern
- fastfetchにインスパイアされた洗練されたUI
- AIコーディングエージェント風のアニメーション (Cursor, Windsurf, Codex)
- カラフルな絵文字とターミナルカラー

### 3. Information Rich
RISC-V特有の情報を徹底的に表示：
- Hart count (ハードウェアスレッド数)
- ISA extensions (M, A, F, D, C, V, Zicsr, etc.)
- Board detection (VisionFive 2, SiFive Unmatched, Milk-V, etc.)
- Vector extension details
- Cache hierarchy (L1D, L1I, L2)
- SoC information from device tree

### 4. Professional Quality
- Rustで実装 (速度、安全性)
- 包括的なテスト
- CI/CDパイプライン
- pre-commit hooks
- クリーンなコード構造

## 🏗️ Architecture

### Module Structure

```
src/
├── main.rs       # Entry point, CLI parsing, orchestration
├── info.rs       # System information gathering
├── display.rs    # Logo and animation display
└── benchmark.rs  # ISA-specific benchmarks
```

### Information Sources

| Information | Source |
|------------|--------|
| Hart count | `/proc/cpuinfo` (processor entries) |
| CPU/ISA | `/proc/cpuinfo` (isa field) |
| Board model | `/proc/device-tree/model` |
| SoC | `/proc/device-tree/compatible` |
| Cache | `/sys/devices/system/cpu/cpu0/cache/` |
| Memory | `sysinfo` crate |
| Kernel | `uname -r` |
| OS | `/etc/os-release` |
| Uptime | `sysinfo::System::uptime()` |

## 🎯 Target Users

1. **RISC-V Enthusiasts**: 自分のセットアップをSNSで共有したい
2. **Developers**: RISC-V開発環境の確認
3. **Board Vendors**: 製品のデモ・マーケティング
4. **Educators**: RISC-Vアーキテクチャの教育

## 🚀 Future Vision

### Short Term (v0.2.0)
- [ ] More board recognition (BeagleV, Allwinner D1)
- [ ] VLEN/ELEN detection for Vector extension
- [ ] JSON output mode
- [ ] Custom color schemes

### Medium Term (v0.3.0)
- [ ] Screenshot generation for SNS
- [ ] Compare mode (vs x86_64/ARM64)
- [ ] Privilege level detection (M/S/U)
- [ ] Performance counter integration

### Long Term (v1.0.0)
- [ ] Community logo contributions
- [ ] Plugin system for custom info
- [ ] Integration with RISC-V International
- [ ] Become the de-facto RISC-V info tool

## 📊 Success Metrics

- GitHub stars: Target 100+ (indicates community interest)
- Weekly downloads: Target 500+ (indicates actual usage)
- Mentioned in RISC-V International blog/social media
- Included in major RISC-V distributions (Debian RISC-V, Fedora RISC-V)
- Board vendors using it in demos (StarFive, SiFive, etc.)

## 🤝 Community Strategy

1. **Launch**: Announce on Reddit r/RISCV, RISC-V mailing lists
2. **Engage**: Respond to issues, accept PRs for board detection
3. **Promote**: Submit to RISC-V Weekly, RISC-V International
4. **Collaborate**: Work with board vendors for accurate detection
5. **Expand**: Accept community logo designs

## 📝 License & Attribution

- **License**: MIT (kako-jun)
- **Inspired by**: fastfetch (modern approach), neofetch (pioneering)
- **For**: RISC-V community worldwide

---

**Last Updated**: 2024-11-17
**Status**: Initial Release Ready
**Version**: 0.1.0
