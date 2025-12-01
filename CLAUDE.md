# riscfetch 開発者向けドキュメント

RISC-V専用のシステム情報表示ツール。

## コンセプト

**世界初のRISC-V専用fetchツール**として、fastfetch/neofetchとは**競合せず補完する**存在：
- 汎用情報（Shell, Terminal, Packages等）はfastfetchに任せる
- RISC-V特化情報（Hart, ISA拡張, ボード検出等）をriscfetchで提供

### RISC-V以外での動作

RISC-V以外では `Sorry, not RISC-V 😢` と表示して終了。これは意図的な設計。

## プロジェクト構造

```
src/
├── main.rs       # CLIパース、オーケストレーション
├── info.rs       # システム情報収集
├── display.rs    # ロゴ、アニメーション表示
└── benchmark.rs  # ISA固有ベンチマーク
```

## 依存クレート

- `clap` 4.5: CLI引数パース
- `colored` 2.1: ターミナルカラー出力
- `sysinfo` 0.31: クロスプラットフォームシステム情報

## 情報ソース

| 情報 | ソース |
|------|--------|
| Hart数 | `/proc/cpuinfo` (processor entries) |
| CPU/ISA | `/proc/cpuinfo` (isa field) |
| ボードモデル | `/proc/device-tree/model` |
| SoC | `/proc/device-tree/compatible` |
| キャッシュ | `/sys/devices/system/cpu/cpu0/cache/` |
| メモリ | `sysinfo` crate |
| カーネル | `uname -r` |
| OS | `/etc/os-release` |

## 主要な設計判断

### ADR-001: Rust採用
- 性能（C/C++並み）、安全性、シングルバイナリ配布
- RISC-Vへの一級サポート

### ADR-002: fastfetchとの補完関係
- 汎用情報の重複を避ける
- RISC-V特化の価値を明確化
- 小さなスコープ = メンテナンス容易

### ADR-007: スプラッシュアニメーションはオプトイン
- `--splash`フラグで有効化
- デフォルトは高速情報表示

### ADR-010: RISC-V以外はハードフェイル
- 明確なツールアイデンティティ
- フォールバックロジック不要
- 差別化要因としての排他性

### ADR-011: ボード検出はDevice Tree経由
- `/proc/device-tree/model`と`compatible`を使用
- 既知ボードパターンをハードコード

## ボード検出パターン

| ベンダー | Compatible文字列 | 表示名 |
|----------|-----------------|--------|
| StarFive | starfive,visionfive2 | StarFive VisionFive 2 |
| SiFive | sifive,hifive-unmatched | SiFive HiFive Unmatched |
| Milk-V | milkv,mars | Milk-V Mars |
| T-Head | thead,* | T-Head Board |

## ISA拡張パース

```
入力: "rv64imafdcv_zicsr_zifencei_zba_zbb"

1. ベース抽出: "rv64" または "rv32"
2. 標準文字抽出: i, m, a, f, d, c, v
3. '_'で分割してZ拡張取得
4. 文字を説明にマッピング:
   - M: "M (Multiply)"
   - A: "A (Atomic)"
   - F: "F (Float)"
   - D: "D (Double)"
   - C: "C (Compressed)"
   - V: "V (Vector)"
```

## ビルド・テスト

```bash
cargo build           # デバッグビルド
cargo build --release # リリースビルド
cargo test            # テスト実行
cargo clippy          # Lint
cargo fmt             # フォーマット
```

## CI/CD

- **ci.yml**: Push/PR時にテスト、フォーマット、Lint
- **release.yml**: タグプッシュで複数プラットフォームビルド
  - Linux x86_64 (glibc, musl)
  - macOS x86_64, ARM64
  - Windows x86_64

## 将来の拡張

### v0.2.0
- 追加ボード検出（BeagleV, Allwinner D1）
- VLEN/ELEN検出
- JSON出力モード

### v0.3.0
- スクリーンショット生成（SNS共有用）
- 権限レベル検出（M/S/U）
- パフォーマンスカウンター統合

### v1.0.0
- i18n対応
- コミュニティボードデータベース
- 主要RISC-Vディストリビューションへのパッケージ化
