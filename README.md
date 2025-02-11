# Downloadフォルダを綺麗に保とう！

ダウンロードフォルダに24時間以上滞在しているファイル全削除君。(Mac専用)

## インストール

```shell
$ git clone https://github.com/uekann/download_cleaner.git
$ cd download_cleaner
$ cargo build --release
$ ln -s $(realpath target/release/download_cleaner) /usr/local/bin/download_cleaner
$ cp com.uekann.DownloadCleaner.plist ~/Library/LaunchAgents
```

## 仕様

エージェントに追加した時から1時間おきに実行される。
取得された最終更新時から24時間以上経過しているファイル・ディレクトリを問答無用で全部消す。
`/tmp/download_cleaner.out.log`と`/tmp/download_cleaner.err.log`にログが吐かれる。
