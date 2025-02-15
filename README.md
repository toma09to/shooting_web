# shooting_web
Processingで作成した[シューティングゲーム](https://github.com/toma09to/Processing/tree/main/shooting)のWeb版です。
クライアント・サーバー間の通信はWebSocketを使用しています。

## Usage
サーバーに関する設定は`server/.env`に記述します。
使用するポート番号を`8080`から変更する場合は`PORT`の値を変更してください。
プロキシにより、クライアント側から見たエンドポイントがSSL/TLSにより暗号化されている場合は`USE_SSL`を`true`にしてください。
```
USE_SSL=false
PORT=8080
```

### Use Docker
Requirements:
- [Docker](https://www.docker.com)

```sh
$ docker build -t shooting_server .
$ docker run -it --rm --name shooting_server shooting_server
```

### Execute directly
Requirements:
- [Rust](https://www.rust-lang.org)

```sh
$ cd server
$ cargo install --path .
$ shooting_server
```
