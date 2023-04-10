# セットアップ

このプロジェクトを動作させるには次を手順を実行ください。

```
python -m venv .venv
source .venv/bin/activate

# 仮想環境に<my_rust_sample>パッケージをインストールする
maturin develop

# 疎通確認
python main.py
```


# 初めからセットアップ

maturin を使用して、rust/pyo3 をセットアップするには次の手順を参考にしてください。

```
# pyo3のrustプロジェクトを作成し、疎通確認用コードを生成
maturin new my_rust_sample

# 仮想環境を用意する（maturinで開発する場合に必要）
python -m venv .venv
source .venv/bin/activate

# 仮想環境に<my_rust_sample>パッケージをインストールする
maturin develop

# rustのライブラリをpythonから呼び出す
python -c "import my_rust_sample; print(my_rust_sample.sum_as_string(1, 2))"
```


# 未解決事項

- pyo3でpyiを自動生成することはできない。将来の自動生成に期待。



# 概要

rust を覚えるためのプロジェクト。

# 基礎教養

## パッケージマネージャ

- rustc: コンパイラ
- rustup: rust本体に関することや、コンパイルや開発に必要なツール群を管理。
- cargo: パッケージマネージャ。crates.io を参照する。

## ライブラリ

- rustfmt: フォーマッター
- clippy: 静的解析
- test: テストツールは純正のものが初めからある

## 言語

- src/lib.rs: ライブラリのエントリポイントとなるようだ？？


