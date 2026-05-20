# calculator_rs

Rust で作る教育用電卓アプリ。

字句解析(Lexer)・構文解析(Parser)・評価(Evaluator) という言語処理系の基礎パイプラインを学ぶことを目的としている。

## ビルド・実行

```bash
cargo run
```

## 機能

- 四則演算（`+` `-` `*` `/`）
- 演算子優先順位・カッコ
- 負の数・小数
- 組み込み関数（`sqrt`・`pow`・`log`）
- メモリスロット（M1〜M5）

## 技術スタック

- Rust
- egui / eframe（GUI）
