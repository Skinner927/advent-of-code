import re

entry_reg = re.compile(r'\[\d+-(\d+)-(\d+) (\d{2}):(\d{2})\] (.+)')
guard_reg = re.compile(r'Guard #(\d+) begins shift')

guard_times = dict()

# ID, Time asleep
current_guard = [None, None]


def process_line(line):
    """
    Read each sorted line and fill our guard_times dict
    :param str line:
    """
    global current_guard

    if not line:
        return
    line = line.strip()
    m = entry_reg.match(line)
    if not m:
        return

    # The challenge only cares about minutes
    # month = int(m.group(1))
    # day = int(m.group(2))
    hour = int(m.group(3))
    minute = int(m.group(4))
    state = m.group(5)

    if state == 'falls asleep':
        if hour != 0:
            raise ValueError('Not operating on 0 hour!')
        # When we fall asleep, just mark the time we fall asleep
        current_guard[1] = minute
    elif state == 'wakes up':
        if hour != 0:
            raise ValueError('Not operating on 0 hour!')
        # When we wake up, update from the stored fall sleep time up to,
        # but not including our time
        for i in time_gen(current_guard[1], minute):
            guard_times[current_guard[0]][i] += 1
        current_guard[1] = None
    else:
        m = guard_reg.match(state)
        if not m:
            print('Cannot read guard state {} from {}'.format(state, line))
            return None
        guard_id = m.group(1)
        current_guard = [guard_id, None]

        if guard_id not in guard_times:
            # Count number of times asleep in the given hour
            guard_times[guard_id] = [0] * 60


def time_gen(start, end):
    """

    :param int start: inclusive
    :param int end: exclusive
    :return:
    """
    if end < start:
        for x in range(start, 60):
            yield x
        for x in range(0, end + 1):
            yield x
    else:
        for x in range(start, end):
            yield x


def main():
    with open('data.txt', 'r') as f:
        for line in sorted(f.readlines()):
            if line:
                process_line(line)
        # data = [process_lines(l) for l in sorted(f.readlines()) if l]

    # PART 1
    # Minutes asleep, guard ID, times
    most_asleep = [0, None, None]
    for guard, times in guard_times.items():
        asleep = sum(times)
        if asleep > most_asleep[0]:
            most_asleep = [asleep, guard, times]

        print('ID: {}'.format(guard))
        print(' '.join('{:02d}'.format(x) for x in range(60)))
        print(' '.join('{:2d}'.format(x) for x in times) + ' %s' % asleep)
        print()

    asleep, guard, times = most_asleep
    print('Most asleep {} with {}'.format(guard, asleep))
    # Sort times by largest
    max_hour = sorted(
        ((c, hr) for hr, c in enumerate(times)),
        reverse=True)[0][1]
    val = max_hour * int(guard)
    print('max hour {} x (guard){} = {}'.format(max_hour, guard, val))

    # PART 2
    # Minutes asleep, time minute, guard
    max_minute = (0, None, None)
    for guard, times in guard_times.items():
        win = max((t, i) for i, t in enumerate(times))
        my_max = win + (guard,)
        if my_max > max_minute:
            max_minute = my_max

    print()
    print('Part2: min asleep, time minute, guard')
    print(max_minute)
    print(max_minute[1] * int(max_minute[2]))


main()
