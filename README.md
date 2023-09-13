# 手を動かして考えればよくわかる 高効率言語 Rust 書きかた・作りかた

ISBN: 978-4-8026-1351-4

電子版発行日: 2022年3月25日 初版第1刷発行

Amazon: https://www.amazon.co.jp/dp/4802613512

写経期間: 2023/08/19 ~ 2023/09/13

## 動かし方

Visual Studio Code の Devcontainerで動作

Docker Desktop | Rancher Desktop のどちらかまたは互換性のあるものをいれる

## キーワード

- python + rust
  - pyo3: https://docs.rs/pyo3/latest/pyo3/
  - cdll: https://docs.python.org/ja/3/library/ctypes.html
- wasm
  - wasm-pack / wasm-bindgen

## 書籍上のミス等

### P.522

誤: `from ctypes import`

```py
# Pythonで動的ライブラリーを利用する
import platform, os
from ctypes import
```

正: `from ctypes import *`

```py
# Pythonで動的ライブラリーを利用する
import platform, os
from ctypes import *
```

### P.545

誤: `let mut file = file::create(filename).unwrap();`

正: `let mut file = File::create(filename).unwrap();`


```diff
    let sel = Selector::parse(".articles img").unwrap();
    for (i, node) in doc.select(&sel).enumerate() {
        let src = node.value().attr("src").unwrap();
        let img_url = format!("{}/{}", shodou_url, src);
        println!("{}", img_url);
        let filename = format!("shodou_{}_{}.png", title, i);
        let bytes = reqwest::get(img_url).await.unwrap().bytes().await.unwrap();
-        let mut file = file::create(filename).unwrap();
+        let mut file = File::create(filename).unwrap();
        file.write_all(&bytes).unwrap();
        time::sleep(time::Duration::from_millis(1000)).await;
    }
}
```

### P.549

block-modes等の最新版を入れると上手く動かないのでバージョンを書籍で指定されているものに固定
