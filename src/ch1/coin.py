price = 3950
coin_500 = 10
coin_100 = 3
coin_50 = 10

for i_500 in range(0, coin_500 + 1):
    for i_100 in range(0, coin_100 + 1):
        for i_50 in range(0, coin_50 + 1):
            total = 500 * i_500 + 100 * i_100 + 50 * i_50
            if (total == price):
                print("500円x{}枚, 100円x{}枚, 50円x{}枚".format(i_500, i_100, i_50))
