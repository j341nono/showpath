# showpath

`showpath` は、Zsh 環境での作業効率を高めるために作られた、軽量で便利な CLI ツールです。  
PATH の管理や `.zshrc` の確認・検索・バックアップなど、シェル環境の見直しやトラブルシューティングに役立ちます。

---

## 📌 このアプリについて

`showpath` は、以下のような場面で活躍する Rust 製の CLI ユーティリティです：

- `$PATH` の中身を見やすく一覧表示したい
- 重複している PATH を検出したい
- `.zshrc` の内容をすぐに確認したい
- 特定の設定やエイリアスを検索したい
- `.zshrc` を定期的にバックアップしたい
- 環境変数を簡単に確認したい

---

## 💡 開発背景

Zsh を日常的に使用する中で、以下のような不便を感じていました：

- `$PATH` を確認するたびに `echo $PATH` を打つのが面倒
- どの PATH が重複しているのか気付かずに環境が煩雑になりがち
- `.zshrc` を編集・確認・検索したい時に、エディタを開くのが手間
- バックアップを取るのを忘れて `.zshrc` を壊すリスクがある

これらを **1つのツールで解決できるようにしたい** という思いから、このツールを開発しました。

---

## 🛠️ 主な機能

| オプション | 説明 |
|-----------|------|
| *(なし)* | `$PATH` の内容を一覧表示 |
| `--duplicates` | 重複している `$PATH` のエントリを検出・表示 |
| `--env` | すべての環境変数を表示（`--env`）または指定した変数のみ表示（例：`--env PATH`） |
| `--zshrc` | `~/.zshrc` の内容を行番号付きで表示 |
| `--zshrc-search <KEYWORD>` | `.zshrc` からキーワードを検索し、該当行と行番号を表示 |
| `--zshrc-backup` | `.zshrc` を日付付きファイルにバックアップ（例：`.zshrc.bak.20250616`） |

---

## 🚀 使用方法

PATH の表示（デフォルト）
$ showpath

重複PATHの表示
$ showpath --duplicates

環境変数の表示
$ showpath --env # すべて表示
$ showpath --env PATH # 特定の変数だけ表示

.zshrc の操作
$ showpath --zshrc
$ showpath --zshrc-search alias
$ showpath --zshrc-backup

yaml
コードをコピーする

---

## 🧪 使用技術

- **Rust**（高速・安全なシステムプログラミング言語）
- [`clap`](https://crates.io/crates/clap)（コマンドライン引数の処理）
- [`chrono`](https://crates.io/crates/chrono)（日付・時刻の操作）
- Rust 標準ライブラリ（`env`, `fs`, `path` など）

---

## 📦 ビルド方法

Rust がインストールされていれば、以下でビルドできます：

cargo build --release

yaml
コードをコピーする

生成されたバイナリは `target/release/showpath` に出力されます。

---

## 📝 ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。

---

## 🙌 貢献

バグ報告・機能要望・改善提案は Issue または Pull Request にて歓迎します！