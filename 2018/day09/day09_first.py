from blist import blist
import itertools


# class IndexConstrain(object):
#     def __init__(self, constrain):
#         self.constrain = constrain
#
#     def __call__(self, i):
#         if i < 0:
#             return self(len(self.constrain) + (i % (len(self.constrain))))
#         if i >= len(self.constrain):
#             return self(i % len(self.constrain))
#         return i


def index_constrain(length, i):
    if i < 0:
        return index_constrain(length, length + (i % length))
    if i >= length:
        return index_constrain(length, i % length)
    return i


def test_index_constrain():
    a = [1, 2, 3]

    assert index_constrain(len(a), 0) == 0
    assert index_constrain(len(a), 1) == 1
    assert index_constrain(len(a), 3) == 0
    assert index_constrain(len(a), -1) == 2
    assert index_constrain(len(a), 6) == 0
    assert index_constrain(len(a), 5) == 2
    assert index_constrain(len(a), -10) == 2

    a.append(4)
    assert index_constrain(len(a), 0) == 0
    assert index_constrain(len(a), 1) == 1
    assert index_constrain(len(a), 3) == 3
    assert index_constrain(len(a), 4) == 0
    assert index_constrain(len(a), -1) == 3
    assert index_constrain(len(a), 6) == 2
    assert index_constrain(len(a), 5) == 1
    assert index_constrain(len(a), -10) == 2
    assert index_constrain(len(a), -7) == 1


def insert_marble(arr, current, value):
    y = index_constrain(len(arr), current + 2)
    if y == 0:
        arr.append(value)
        return len(arr) - 1
    arr.insert(y, value)
    return y


def test_insert_marble():
    a = [1, 2, 3, 4, 5]
    assert insert_marble(a, 0, 9) == 2
    assert a == [1, 2, 9, 3, 4, 5]

    a = [1, 2, 3, 4, 5]
    assert insert_marble(a, 4, 9) == 1
    assert a == [1, 9, 2, 3, 4, 5]

    a = [1, 2, 3]
    assert insert_marble(a, 3, 9) == 2
    assert a == [1, 2, 9, 3]


def run(num_players, num_marbles):
    player_score = [0] * num_players

    board = blist([0])
    marble = 0
    current = 0

    for player in itertools.cycle(range(num_players)):
        marble += 1
        if marble > num_marbles:
            break

        if marble % 23 == 0:
            player_score[player] += marble
            # Remove the marble 7 index counter clockwise and add to our score
            remove = index_constrain(len(board), current - 7)
            player_score[player] += board.pop(remove)
            current = index_constrain(len(board), remove)
        else:
            # Regular play
            current = insert_marble(board, current, marble)

    return max(*player_score)


def main():
    # Examples given
    # assert run(10, 1618) == 8317
    # assert run(13, 7999) == 146373
    # assert run(30, 5807) == 37305

    # Input test data:
    # 416 players; last marble is worth 71975 points
    num_players = 416
    last_marble = 71975 * 100  # Drop the * 100 to get the first run's result

    print(run(num_players, last_marble))


test_index_constrain()
test_insert_marble()
main()
