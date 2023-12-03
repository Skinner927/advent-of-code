import sys
import re
import pprint
import heapq
import string

req_match = re.compile(r'Step ([A-Z]) must be finished before step '
                       r'([A-Z]) can begin\.')


class Worker(object):
    def __init__(self, id, task, time_left):
        self.id = id
        self.task = task
        self.time_left = time_left

    def __str__(self):
        return 'Worker {}: {} @ {}'.format(self.id, self.task, self.time_left)


def main():
    # Key has list of children
    steps = dict()
    depends = dict()  # Inverse of steps
    build_time = {k: 61 + i for i, k in enumerate(string.ascii_uppercase)}

    with open('data.txt', 'r') as f:
        for line in (l.strip() for l in f.readlines()):
            if not line:
                continue
            m = req_match.match(line)
            if not m:
                print('Could not match line: {}'.format(line))
                return 1
            parent = m.group(1).upper()
            child = m.group(2).upper()

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

    print('Steps')
    pprint.pprint(steps, indent=2)
    # print_graph(steps)
    print('Depends')
    pprint.pprint(depends, indent=2)

    # Find nodes without dependencies. These become our starting set.
    starters = []
    for key, depends_on in depends.items():
        if len(depends_on) == 0:
            starters.append(key)

    print('Starters')
    print(starters)

    print('Build times')
    print(build_time)

    NUM_WORKERS = 5
    workers = [Worker(i, None, 0) for i in range(NUM_WORKERS)]
    time_elapsed = 0

    # Heapq work queue because we need to do work alphabetically
    work = list(starters)
    heapq.heapify(work)

    route = []
    being_worked_on = set()
    task_count = len(list(steps.keys()))
    print('Time  Workers   Work')
    while len(route) < task_count:
        msg = '{:04d} '.format(time_elapsed)
        msg += ' '.join(
            '{}'.format(w.task if w.task else '-') for w in workers)
        print('{}  {}'.format(msg, work))
        # Figure out how much time we need to wait until a worker is done
        min_work_left = 0
        for worker in workers:
            if worker.task and worker.time_left > 0:
                if not min_work_left or worker.time_left < min_work_left:
                    min_work_left = worker.time_left

        # Do all work
        if min_work_left:
            time_elapsed += min_work_left
            for worker in workers:
                # Reduce time to finish
                if worker.task and worker.time_left > 0:
                    worker.time_left -= min_work_left
                # Finish task if it's done
                if worker.task and worker.time_left < 1:
                    # print('Worker {} done with {} @ {}'
                    #       ''.format(worker.id, worker.task, time_elapsed))
                    route.append(worker.task)
                    # Push this node's children into the work heap
                    for child in steps[worker.task]:
                        if child not in work:
                            heapq.heappush(work, child)
                    worker.task = None

        # Assign work to free workers
        free_workers = [w for w in workers if w.task is None]
        work_to_add_back = []
        while len(work) > 0 and len(free_workers) > 0:
            task = heapq.heappop(work)
            if task in route or task in being_worked_on:
                # We've already built this task
                continue

            # Ensure we've met all our dependencies
            skip = False
            for dependency in depends[task]:
                if dependency not in route:
                    skip = True
                    break
            if skip:
                # We'll add it back as it needs to be done but it
                # can't be done yet
                work_to_add_back.append(task)
                continue

            # At this point we have a task we can do
            # Always pop the first worker (FIFO)
            worker = free_workers.pop(0)

            worker.task = task
            worker.time_left = build_time[task]
            being_worked_on.add(task)

        # Add back all work
        for task in work_to_add_back:
            heapq.heappush(work, task)

    print('\ndone')
    print(''.join(route))
    print('time: {}'.format(time_elapsed))

    return 0


sys.exit(main())
