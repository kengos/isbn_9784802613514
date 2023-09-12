## Crates

- https://docs.rs/pyo3/latest/pyo3/

## Python

- cdll: https://docs.python.org/ja/3/library/ctypes.html

## Misprints

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
