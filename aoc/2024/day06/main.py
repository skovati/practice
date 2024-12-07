import pathlib
from typing import TypeAlias
from enum import Enum, auto
from copy import deepcopy

Location: TypeAlias = tuple[int, int]
Floor: TypeAlias = list[list[str]]

class Direction(Enum):
    north = auto()
    south = auto()
    east = auto()
    west = auto()

    def rotate(self):
        match self:
            case Direction.north:
                return Direction.east
            case Direction.south:
                return Direction.west
            case Direction.east:
                return Direction.south
            case Direction.west:
                return Direction.north

    def to_move(self):
        match self:
            case Direction.north:
                return (-1, 0)
            case Direction.south:
                return (1, 0)
            case Direction.east:
                return (0, 1)
            case Direction.west:
                return (0, -1)


class Guard():
    location: Location
    direction: Direction
    seen: set[(int, int)]
    floor: Floor

    def __init__(self, location, direction, floor):
        self.location = location
        self.direction = direction
        self.seen = set()
        self.seen.add(location)
        self.floor = floor

    def legal_step(self, location: Location):
        return location[0] in range(len(self.floor)) and location[1] in range(len(self.floor[0]))

    def step(self):
        #print(f"taking step at location {self.location} in direction {self.direction}")
        step_dir = self.direction.to_move()

        canditate_location = (self.location[0] + step_dir[0], self.location[1] + step_dir[1])
        #print(f"canditate_location: {canditate_location}")

        if not self.legal_step(canditate_location):
            #print("this would step off the map! returning false")
            return False

        if get_tile(self.floor, canditate_location) == "#":
            self.direction = self.direction.rotate()
            return True
        else:
            self.seen.add(canditate_location)
            self.location = canditate_location
            return True

    def infinite_loop(self):
        seen_pos = set()
        curr_pos = (self.location[0], self.location[1], self.direction)
        seen_pos.add(curr_pos)

        while self.step():
            curr_pos = (self.location[0], self.location[1], self.direction)
            if curr_pos in seen_pos:
                return True
            seen_pos.add(curr_pos)

        return False


def get_tile(floor: Floor, location: Location) -> str:
    return floor[location[0]][location[1]]

def parse_map(map: str) -> Guard:
    map = [list(line) for line in map.splitlines()]

    for i, _ in enumerate(map):
        for j, _ in enumerate(map[i]):
            if map[i][j] == "^":
                return Guard((i, j), Direction.north, map)
            elif map[i][j] == ">":
                return Guard((i, j), Direction.east, map)
            elif map[i][j] == "<":
                return Guard((i, j), Direction.west, map)
            elif map[i][j] == "v":
                return Guard((i, j), Direction.south, map)

    raise Exception("No character glyph found, can't continue")

def column(matrix, i):
    return [row[i] for row in matrix]

def solve_p2(path: str):
    contents = pathlib.Path(path).read_text()
    guard = parse_map(contents)

    num_loops = 0

    for i, _ in enumerate(guard.floor):
        for j, _ in enumerate(guard.floor[0]):
            if guard.floor[i][j] == "#":
                continue
            temp_guard = deepcopy(guard)
            temp_guard.floor[i][j] = "#"
            if temp_guard.infinite_loop():
                num_loops += 1

    return num_loops

def solve_p1(path: str):
    contents = pathlib.Path(path).read_text()
    guard = parse_map(contents)

    while guard.step():
        pass

    return len(guard.seen)


def main():
    print(f"p1 example solution: {solve_p1('example.txt')}")
    print(f"p1 final solution: {solve_p1('input.txt')}")
    print(f"p2 example solution: {solve_p2('example.txt')}")
    print(f"p2 final solution: {solve_p2('input.txt')}")


if __name__ == "__main__":
    main()
