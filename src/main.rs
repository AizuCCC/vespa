use vespa::toml_struct::Config;

fn main() {
    let config: Config = toml::from_str(
        r#"
        title = '部誌 vol.1'
        editor = 'editor'

        [front]
        path = 'path/to/file'
        author = "toyama"
        [back]
        path = 'path/to/file'
        author = 'toyama1710'

        [[body]]
        files = ['path/to/file']
        author = '遠'
        start = 'Auto' # 先頭大文字しか受け付けないのどうにかしたい
    "#,
    )
    .unwrap();

    println!("{:?}", config);

    // コマンドライン引数を受け取る
    // ブックのディレクトリを開く
    // book.tomlを読む
    // book.toml内のファイルパスを精査する
    // 見開きページに応じてbodyを並べ替える
    // pdfを出力する
}
