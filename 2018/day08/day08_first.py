def sum_node(data):
    num_children = data.pop(0)
    num_metadata = data.pop(0)

    total = 0
    for _ in range(num_children):
        total += sum_node(data)

    for _ in range(num_metadata):
        total += data.pop(0)

    return total


def main():
    with open('data.txt', 'r') as f:
        data = [int(i) for i in f.readline().strip().split(' ')]

    print(sum_node(data))


main()
