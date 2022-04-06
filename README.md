# vespa
漫研の部誌の編集作業を自動化するツール

## about

下のようなtomlファイルから閲覧用のpdfと印刷用の面付済みのpdfを作成する

```toml
title = "タイトル" # ex) title = "漫研部誌vol.1 テーマ[春]"
editor = "編集の名前"
size = "A5" # 1ページのサイズ A5, A4, B5, B4 を指定

[colophon]
publisher = "発行者"
date_of_issue = 2022-04-05 # 発行日
print = "印刷所"
contact = "@hoge (twitter)" # 連絡先 部長メールアドレスや部のtwitterアドレスとか

[front]
path = "path/to/file"
author = "著者名"
[back]
path = "path/to/file"
author = "著者名"

[[body]]
files = ["path/to/file1", "path/to/file2", "path/to/file3"] # この順にページを配置
author = "著者名"
start = "Left" # 見開きの左側からページを置く file2 と file3 が見開きで印刷される

[[body]]
files = ["path/to/file1", "path/to/file2"]
author = "著者名"
start = "Right" # この場合 file1 と file2 が見開きで配置される

[[body]]
files = ["path/to/file"]
author = "著者名"
start = "Auto" # 自動配置 特に理由がない場合はautoにしておく
```
