def fill_path(x, y, dir, count, wire):
    i = 0
    orig_count = count
    if dir == "R":
        while count:
            wire.add((x + count, y))
            count -= 1
        return x + orig_count, y
    elif dir == "L":
        while count:
            wire.add((x - count, y))
            count -= 1
        return x - orig_count, y
    elif dir == "U":
        while count:
            wire.add((x, y + count))
            count -= 1
        return x, y + orig_count
    elif dir == "D":
        while count:
            wire.add((x, y - count))
            count -= 1
        return x, y - orig_count
    else:
        print(f"UNKNOWN DIR {dir}")
        return x, y


def paths_for_wires(*wires):
    results = []
    for wire in wires:
        wire_path = set()
        x, y = 0, 0
        for instruction in wire.split(","):
            dir = instruction[0]
            count = int(instruction[1:])
            x, y = fill_path(x, y, dir, count, wire_path)
        results.append(wire_path)
    return results


def find_closest_crossing(wire_a, wire_b):
    path_a, path_b = paths_for_wires(wire_a, wire_b)

    shortest_dist = 9999
    for collision in path_a & path_b:
        dist = sum(abs(n) for n in collision)
        if dist < shortest_dist:
            shortest_dist = dist

    return shortest_dist


def run_samples():
    for wire_a, wire_b, expected in [
        ("R8,U5,L5,D3", "U7,R6,D4,L4", 6),
        ("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 159),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        ),
    ]:
        result = find_closest_crossing(wire_a, wire_b)
        pass_or_fail = "PASS" if result == expected else "FAIL"
        print(f"{pass_or_fail}: {result}  expected  {expected}")


def run_input():
    with open("input.txt", "r") as f:
        wire_a = f.readline().strip()
        wire_b = f.readline().strip()

    print(find_closest_crossing(wire_a, wire_b))


# run_samples()
run_input()
