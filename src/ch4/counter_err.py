# p.333 python -m py_compile null_err.py ; ファイル名がおかしい
class Counter:
    value = 0

    def inc(self):
        self.value += 1
        print("value =", self.value)

def count(counter: Counter):
    counter.inc()

a = Counter()
count(a)
count(a)

# 間違った利用
a = None
count(a)
