a = 1
b = 1
print(a)

for _ in range(30):
    print(a + b)
    tmp = a
    a = b
    b = tmp + b
