def sum_node(data):
    num_children = data.pop(0)
    num_metadata = data.pop(0)

    total = 0
    children = [sum_node(data) for _ in range(num_children)]

    # Metadata
    if num_children < 1:
        # No children, just add metadata
        for _ in range(num_metadata):
            total += data.pop(0)
    else:
        # Metadata are actually child indexes
        for _ in range(num_metadata):
            child_index = data.pop(0) - 1
            if len(children) > child_index >= 0:
                total += children[child_index]

    # We are now the nth child.
    return total


def main():
    with open('data.txt', 'r') as f:
        data = [int(i) for i in f.readline().strip().split(' ')]

    print(sum_node(data))


main()
