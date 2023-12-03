def fill_path(x, y, current_count, dir, steps, wire):
    x_mod = 0
    y_mod = 0
    if dir == "R":
        x_mod = 1
    elif dir == "L":
        x_mod = -1
    elif dir == "U":
        y_mod = -1
    elif dir == "D":
        y_mod = 1
    else:
        print(f"UNKNOWN DIR {dir}")
        return x, y, current_count

    for i in range(steps):
        x += x_mod
        y += y_mod
        current_count += 1
        wire[(x, y)] = current_count

    return x, y, current_count


def paths_for_wires(*wires):
    results = []
    for wire in wires:
        wire_path = dict()
        x, y, current_count = 0, 0, 0
        for instruction in wire.split(","):
            dir = instruction[0]
            steps = int(instruction[1:])
            x, y, current_count = fill_path(x, y, current_count, dir, steps, wire_path)
        results.append(wire_path)
    return results


def find_closest_crossing(wire_a, wire_b):
    path_a, path_b = paths_for_wires(wire_a, wire_b)
    path_a = set(path_a.keys())
    path_b = set(path_b.keys())

    shortest_dist = 9999
    for collision in path_a & path_b:
        dist = sum(abs(n) for n in collision)
        if dist < shortest_dist:
            shortest_dist = dist

    return shortest_dist


def find_shortest_wire_crossing(wire_a, wire_b):
    path_a, path_b = paths_for_wires(wire_a, wire_b)

    # print_wire([path_a, path_b])

    keys_a = set(path_a.keys())
    keys_b = set(path_b.keys())

    shortest_dist = 9999999999
    for collision in keys_a & keys_b:
        dist = path_a[collision] + path_b[collision]
        if dist < shortest_dist:
            shortest_dist = dist

    return shortest_dist


def print_wire(wires):
    min_x, max_x, min_y, max_y = 0, 0, 0, 0
    for wire in wires:
        for c in wire.keys():
            if c[0] < min_x:
                min_x = c[0]
            if c[0] > max_x:
                max_x = c[0]
            if c[1] < min_y:
                min_y = c[1]
            if c[1] > max_y:
                max_y = c[1]

    print(f"{(min_x, min_y)} to {(max_x, max_y)}")

    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            has = 0
            for wire in wires:
                if wire.get((x, y), False):
                    has += 1
            if has > 1:
                print("X", end="")
            elif has > 0:
                print(".", end="")
            else:
                print("O" if x == 0 and y == 0 else " ", end="")
        print("")


def run_samples():
    for wire_a, wire_b, expected in [
        ("R8,U5,L5,D3", "U7,R6,D4,L4", 30),
        ("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 610),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            410,
        ),
    ]:
        result = find_shortest_wire_crossing(wire_a, wire_b)
        pass_or_fail = "PASS" if result == expected else "FAIL"
        print(f"{pass_or_fail}: {result}  expected  {expected}")


def run_input():
    with open("input.txt", "r") as f:
        wire_a = f.readline().strip()
        wire_b = f.readline().strip()

    print(find_shortest_wire_crossing(wire_a, wire_b))


# run_samples()
run_input()
