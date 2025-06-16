# showpath

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`showpath` は、Zsh環境での作業効率を劇的に向上させる軽量CLI ツールです。  
PATH管理から`.zshrc`の操作まで、シェル環境の管理に必要な機能を1つのツールに集約しました。

## ✨ 特徴

- 🔍 **直感的なPATH表示** - 見やすい形式でPATHを一覧表示
- 🔄 **重複検出** - 無駄な重複PATHを自動検出
- ⚙️ **設定ファイル管理** - `.zshrc`の表示・検索・バックアップが簡単
- 🌍 **環境変数確認** - 任意の環境変数を素早く確認
- ⚡ **高速動作** - Rust製で軽量・高速

## 🚀 クイックスタート

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/j341nono/showpath.git
cd showpath
cargo build --release
cargo install --path .
```

### 基本的な使用方法

```bash
# PATH を見やすく表示
showpath

# 重複しているPATHを検出
showpath --duplicates

# .zshrc の内容を表示
showpath --zshrc

# 特定の設定を検索
showpath --zshrc-search "alias"

# .zshrc をバックアップ
showpath --zshrc-backup
```

## 📋 使用例

### PATH の表示
```bash
$ showpath
📁 Current PATH entries:
 1. /usr/local/bin
 2. /usr/bin
 3. /bin
 4. /usr/sbin
 5. /sbin
 6. /opt/homebrew/bin
 7. ~/.cargo/bin
```

### 重複検出
```bash
$ showpath --duplicates
⚠️  Duplicate PATH entries found:
 • /usr/local/bin (appears 2 times)
 • /usr/bin (appears 2 times)
```

### .zshrc の検索
```bash
$ showpath --zshrc-search "export"
🔍 Found 3 matches in ~/.zshrc:
 12: export PATH="/opt/homebrew/bin:$PATH"
 25: export EDITOR="nvim"
 38: export LANG="ja_JP.UTF-8"
```

## 🛠️ 全機能リファレンス

| コマンド | 説明 | 使用例 |
|---------|------|--------|
| `showpath` | PATH の内容を見やすく一覧表示 | `showpath` |
| `--duplicates` | 重複PATHエントリを検出・表示 | `showpath --duplicates` |
| `--env [変数名]` | 環境変数を表示 | `showpath --env` <br> `showpath --env PATH` |
| `--zshrc` | `.zshrc`を行番号付きで表示 | `showpath --zshrc` |
| `--zshrc-search <KEYWORD>` | `.zshrc`内でキーワード検索 | `showpath --zshrc-search alias` |
| `--zshrc-backup` | `.zshrc`をバックアップ | `showpath --zshrc-backup` |
| `--help` | ヘルプを表示 | `showpath --help` |
| `--version` | バージョン情報を表示 | `showpath --version` |

## 💡 開発の動機

Zshを日常的に使用する中で感じた以下の課題を解決するために開発しました：

- **PATH管理の煩雑さ** - `echo $PATH`の出力は読みにくく、重複に気づきにくい
- **設定ファイルの管理** - `.zshrc`の確認・編集・バックアップが面倒
- **環境変数の確認** - 特定の環境変数を素早く確認したい
- **作業効率の向上** - 複数のコマンドを1つのツールで済ませたい

## 🏗️ 技術スタック

- **[Rust](https://www.rust-lang.org/)** - 高速・安全・並行性に優れたシステムプログラミング言語
- **[clap](https://crates.io/crates/clap)** - モダンなコマンドライン引数パーサー
- **[chrono](https://crates.io/crates/chrono)** - Rustの日付・時刻ライブラリ
- **標準ライブラリ** - `std::env`, `std::fs`, `std::path`

## 📈 ロードマップ

- [ ] 設定ファイル（`.showpathrc`）のサポート
- [ ] カスタムカラーテーマ
- [ ] JSON/YAML形式での出力オプション
- [ ] 他のシェル（Bash, Fish）のサポート
- [ ] Homebrewでの配布

## 🤝 貢献

プロジェクトへの貢献を歓迎します！

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. Pull Requestを作成

### 開発環境のセットアップ

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/showpath.git
cd showpath

# 依存関係をインストール
cargo build

# テストを実行
cargo test

# 開発用ビルド
cargo run -- --help
```

## 📄 ライセンス

このプロジェクトは[MIT License](LICENSE)の下で公開されています。

## 🙏 謝辞

このツールは以下にインスパイアされました：
- Zshコミュニティの皆様
- Rustエコシステムの開発者の皆様

---

**問題や提案がある場合は、[Issues](https://github.com/yourusername/showpath/issues)でお知らせください！**