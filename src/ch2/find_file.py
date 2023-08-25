import sys, os

if len(sys.argv) < 3:
    print("findfile.py (path) (keyword)")
    quit()

target_dir = sys.argv[1]
keyword = sys.argv[2]

# 指定のディレクトリを検索
for dirname, _, files in os.walk(target_dir):
    for file in files:
        if keyword in file:
          fullpath = os.path.join(dirname, file)
          print(fullpath)
