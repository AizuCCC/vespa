# vespa
漫研の部誌の編集作業を自動化するツール

## about

下のようなtomlファイルから閲覧用のpdfと印刷用の面付済みのpdfを作成する

```toml
title = "タイトル" # ex) title = "漫研部誌vol.1 テーマ[春]"
editor = "編集の名前"

[front]
path = "path/to/file"
author = "著者名"
[back]
path = "path/to/file"
author = "著者名"

[[body]]
files = ["path/to/file1", "path/to/file2", "path/to/file3"] # この順にページを配置
author = "著者名"
start_with_right = false # 見開きの左側からページを置く file2 と file3 が見開きで印刷される

[[body]]
files = ["path/to/file1", "path/to/file2"]
author = "著者名"
start_with_right = true # この場合 file1 と file2 が見開きで配置される

[[body]]
files = ["path/to/file"]
author = "著者名"
# start_with_right のキーが無い場合，自動配置される
# 特に理由が無い限りは指定しない方がよい
```
