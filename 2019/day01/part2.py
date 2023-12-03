fuel_sum = 0


def fuel_req(mass):
    return max(0, (mass // 3) - 2)


with open("./input.txt", "r") as f:
    while True:
        line = f.readline().strip()
        if not line:
            break

        mass = int(line)
        last_fuel_cost = fuel_req(mass)
        while last_fuel_cost > 0:
            fuel_sum += last_fuel_cost
            last_fuel_cost = fuel_req(last_fuel_cost)

print(str(fuel_sum))
