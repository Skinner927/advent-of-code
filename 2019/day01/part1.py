fuel_sum = 0

with open("./input.txt", "r") as f:
    while True:
        line = f.readline().strip()
        if not line:
            break

        mass = int(line)
        fuel_sum += (mass // 3) - 2

print(str(fuel_sum))
