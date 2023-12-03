import sys
import re
import pprint
import heapq
import igraph

req_match = re.compile(r'Step ([A-Z]) must be finished before step '
                       r'([A-Z]) can begin\.')


def print_graph(graph):
    """
    Debug
    :param dict graph:
    :return:
    """
    g = igraph.Graph(directed=True)
    keys = list(graph.keys())

    g.add_vertices(len(keys))
    g.vs['label'] = keys

    for parent, children in graph.items():
        for child in children:
            g.add_edge(keys.index(parent), keys.index(child))

    # https://igraph.org/python/doc/tutorial/tutorial.html#layout-algorithms
    layout = g.layout('kk')
    igraph.plot(g, layout=layout, margin=20)


def main():
    # Key has list of children
    steps = dict()
    depends = dict()  # Inverse of steps

    with open('data.txt', 'r') as f:
        for line in (l.strip() for l in f.readlines()):
            if not line:
                continue
            m = req_match.match(line)
            if not m:
                print('Could not match line: {}'.format(line))
                return 1
            parent = m.group(1)
            child = m.group(2)

            # Add each parent -> child relation to the graph
            if parent not in steps:
                steps[parent] = []
            if child not in steps:
                steps[child] = []

            steps[parent].append(child)

            # We want to store the reverse in our depends graph
            if parent not in depends:
                depends[parent] = []
            if child not in depends:
                depends[child] = []

            depends[child].append(parent)

    # print(starters)
    # print(steps)
    print('Steps')
    pprint.pprint(steps, indent=2)
    # print_graph(steps)
    # print(depends)
    print('Depends')
    pprint.pprint(depends, indent=2)

    starters = []
    for key, depends_on in depends.items():
        if len(depends_on) == 0:
            starters.append(key)
    if len(starters) > 1:
        print('WARNING! Multiple possible start nodes: {}'.format(starters))

    print('Starters?')
    print(starters)

    # Heapq work queue because we need to do work alphabetically
    work = list(starters)
    heapq.heapify(work)

    route = []
    while len(work) > 0:
        node = heapq.heappop(work)
        if node in route:
            # We've been here
            continue

        # Ensure we've met all our dependencies
        skip = False
        for dependency in depends[node]:
            if dependency not in route:
                skip = True
                break
        if skip:
            continue

        route.append(node)
        # Push this node's children into the work heap
        for child in steps[node]:
            heapq.heappush(work, child)

    print(''.join(route))

    return 0


sys.exit(main())
