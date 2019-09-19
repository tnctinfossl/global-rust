# システム要求
+ Rustの実行環境
+ GrSimが動作すること
+ goolgle protcol buffer のコンパイラが動くこと
+ Linux (筆者の環境はXUnutu 18.04 LTS))

# インストール
## rust
`curl https://sh.rustup.rs -sSf | sh` --- 自動化できる方法が有ればおしえてください
## パッケージ
参考までに
```
sudo apt update
sudo apt upgrade -y
#google protcol buffer
sudo apt install libprotobuf-dev libprotoc-dev protobuf-compiler
#for azul
sudo apt install cmake libxcb-xkb-dev libfontconfig1-dev libgles2-mesa-dev \
    libfreetype6-dev libexpat-dev
```

## 入れとくと便利
```
sudo apt update
sudo apt upgrade -y
sudo apt install libssl-dev -y
cargo install cargo-edit
cargo install cargo-bench
```

# 設定
初回起動時にsettings.jsonが作られるのでそれを弄ってください。

# 起動
`cargo run`か`cargo build`してできた生成物を実行すること