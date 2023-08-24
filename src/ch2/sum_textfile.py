import sys
total = 0
for i, v in enumerate(sys.argv):
    if i == 0:
        continue
    with open(v, "rt") as fp:
        text = fp.read()
        # 改行で区切って読む
        for line in text.split("\n"):
            try:
                total += float(line)
            except ValueError:
                pass
print(total)
