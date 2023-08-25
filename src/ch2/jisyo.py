import sys
dicfile = "ejdict-hand-utf8.txt"

if len(sys.argv) < 2:
    print("[USAGE] jisyo.py word")
    quit()

word = sys.argv[1]

with open(dicfile, "rt", encoding="utf-8") as fp:
    while True:
        line = fp.readline()
        if not line: break
        if word in line:
            print(line.strip())
