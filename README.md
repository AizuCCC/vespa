# vespa
漫研の部誌の編集作業を自動化するツール

## about

下のようなtomlファイルを使って原稿を取りまとめ、コピー本として印刷するためのpdfを生成する
作られるpdfは以下の3種類

- `book.pdf`
  - PC上での閲覧用のpdf
  - pdfでの1ページ = 本の1ページ なので、見開きを見るのは厳しい
- `cover.pdf`
  - 印刷用の表紙部分
  - 表紙,裏表紙,目次,奥付が並ぶ
- `body.pdf`
  - 印刷用の本文
  - 面付してあるので、そのまま両面印刷できる

```toml
title = "タイトル" # 本の名前 例)title = "漫研部誌vol.1 テーマ[春]"
editor = "編集の名前"

publisher = "発行者" # サークル名(代表者) とかでいいと思う
date_of_issue = 2022-04-05 # 発行日 YYYY-MM-DD 形式で表記
print = "印刷所"
contact = "@hoge (twitter)" # 連絡先 部長メールアドレスや部のtwitterIDとか

size = "A5" # 1ページのサイズ A5, A4, B5, B4 を指定

[front] # 表紙
path = "path/to/file"
author = "著者名"

[back] # 裏表紙
path = "path/to/file"
author = "著者名"

[[body]]
files = ["path/to/file1", "path/to/file2", "path/to/file3"] # この順にページを配置
author = "著者名"
start = "Left" # 見開きの左側からページを置く file2 と file3 が見開きで印刷される

[[body]]
files = ["path/to/file1", "path/to/file2"]
title = "題名"
author = "著者名"
start = "Right" # この場合 file1 と file2 が見開きで配置される

[[body]]
files = ["path/to/file"]
author = "著者名"
start = "Auto" # 自動配置 特に理由がない場合はautoにしておく
```

## インストール方法

まずは Rust をインストールしてください
[参考](https://www.rust-lang.org/ja/tools/install)

次にディレクトリ`vespa`内で次のコマンドを実行してください
```
$ cargo install --path .
```

## 使い方

### pdf生成
原稿をまとめているディレクトリ内で次のコマンドを実行してください
```
$ vespa
```
`./book.toml` を参照してpdfを生成します
同名のpdfが存在している場合、上書きしてしまうので気を付けてください

### book.tomlの書き方

細かい注意点を列挙します

- `start` は `Right`, `Left`, `Auto` のみを受け付ける
  - 大文字小文字に注意してください `right` などは受け付けません
- [[body]] の `[]` は 2 つ重ねる

## 対応しているファイル形式

- 画像ファイル
  - `png`
  - `jpg`

文章については現時点では対応していない

## 綴じ方

漫研なので右綴じ固定
