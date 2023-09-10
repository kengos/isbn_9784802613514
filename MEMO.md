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
