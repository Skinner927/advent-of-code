import itertools
import string
import math

group_id = itertools.product(string.ascii_uppercase, repeat=2)

grid = dict()
grid_b = dict()

min_x = min_y = 90000
max_x = max_y = 0

with open('data.txt', 'r') as f:
    line_split = (l.split(', ') for l in f.readlines())
    coords = []
    for x, y in line_split:
        x = int(x)
        y = int(y)
        coords.append((x, y))

        # Find bounds
        min_x = min(min_x, x)
        max_x = max(max_x, x)
        min_y = min(min_y, y)
        max_y = max(max_y, y)

offset = (min_x - 1, min_y - 1)  # Subtract every XY by this
min_pos = (0, 0)
max_pos = (max_x - offset[0] + 1, max_y - offset[1] + 1)

# Initialize
for y in range(max_pos[1]):
    for x in range(max_pos[0]):
        grid.setdefault(y, dict())[x] = None
        grid_b.setdefault(y, dict())[x] = None

# Jam all coords into the grid/board/graph/whatever
groups = dict()
for x, y in coords:
    x -= offset[0]
    y -= offset[1]
    group_name = ''.join(next(group_id))
    grid[y][x] = group_name
    grid_b[y][x] = group_name
    groups[group_name] = (x, y)


def print_grid(grid):
    for y in grid.values():
        print(' '.join(v if v else '_ ' for v in y.values()))


def manhattan(p1, p2):
    return abs(p2[0] - p1[0]) + abs(p2[1] - p1[1])


# PART 1
# Now let's visit every single empty cell and figure out where it belongs
for y in range(max_pos[1]):
    for x in range(max_pos[0]):
        cell = (x, y)
        if grid[y][x]:
            # Skip if we've got something here (prob a group)
            continue

        shortest = []
        for group_name, pos in groups.items():
            shortest.append((manhattan(cell, pos), group_name))
        shortest.sort()
        if len(shortest) >= 2:
            # Tie?
            if shortest[0][0] == shortest[1][0]:
                grid[y][x] = '.'
                continue
        # else
        grid[y][x] = shortest[0][1]

# print_grid(grid)

# Find all the outside edge groups so we don't count their area
rm = {'.', '_'}
for x in range(max_pos[0]):
    rm.add(grid[0][x])
    rm.add(grid[max_pos[1] - 1][x])
for y in range(max_pos[1]):
    rm.add(grid[y][0])
    rm.add(grid[y][max_pos[0] - 1])

# Find the biggest
areas = dict()
max_area = 0
for y in range(max_pos[1]):
    for x in range(max_pos[0]):
        cell = grid[y][x]
        if cell in rm:
            continue
        if cell not in areas:
            areas[cell] = 0
        areas[cell] += 1
        max_area = max(max_area, areas[cell])

print('MAX: {}'.format(max_area))

# Part 2!!
MAX_DIST = 10000

area_count = 0
# Visit every single empty cell and figure out its manhattan from every group
for y in range(max_pos[1]):
    for x in range(max_pos[0]):
        cell = (x, y)

        d = sum(manhattan(cell, pos) for pos in groups.values())
        if d < MAX_DIST:
            area_count += 1
            grid_b[y][x] = '#'
        else:
            grid_b[y][x] = '.'

# print_grid(grid_b)

print('Part2 Area: {}'.format(area_count))
