import typing

from . import get_input


DIAL_SIZE = 100


class Dial:
    def __init__(self):
        self.n = 50
        self.zeros = 0

    def rotate(self, twist: int):
        self.n += twist
        if self.n % DIAL_SIZE == 0:
            self.zeros += 1

    def do_all(self, turns: list[int]) -> int:
        for turn in turns:
            self.rotate(turn)
        return self.zeros


def get_instructions(input: typing.TextIO) -> list[int]:
    turns = list()
    for line in input:
        line = line.strip()
        if not line:
            continue  # skip empty lines
        if line[0] == "R":
            turns.append(int(line[1:]))
        elif line[0] == "L":
            turns.append(int(line[1:]))
        else:
            complaint = f"Bad line: {line}"
            raise ValueError(complaint)
    return turns


###### TESTS
_TEST_INPUT = """
L68
L30
R48
L5
R60
L55
L1
L99
R14
L282
"""


def test_day1_pt1():
    import io

    input_file = io.StringIO(_TEST_INPUT)
    turns = get_instructions(input_file)
    d = Dial()
    zeros = d.do_all(turns)
    assert zeros == 3


### Main

if __name__ == "__main__":
    input_file = get_input("input_d1.txt")
    d = Dial()
    with input_file.open() as fin:
        turns = get_instructions(fin)
    zeros = d.do_all(turns)
    print(f"Day 1 part 1: {zeros}")
