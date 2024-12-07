input = open("./inputs/day6.txt").read().split("\n")

grid = [list(line) for line in input]


def solve():
    y, x = 0, 0

    for r, row in enumerate(grid):
        for c, cell in enumerate(row):
            if cell == "^":
                y, x = r, c

    positions = {(y, x)}
    delta = (-1, 0)
    while True:
        nextY, nextX = y + delta[0], x + delta[1]
        if nextY >= len(grid) or nextY < 0 or nextX >= len(grid[0]) or nextX < 0:
            break
        elif grid[nextY][nextX] == "#":
            if delta == (-1, 0):
                delta = (0, 1)
            elif delta == (0, 1):
                delta = (1, 0)
            elif delta == (1, 0):
                delta = (0, -1)
            else:
                delta = (-1, 0)
        else:
            y, x = nextY, nextX
            positions.add((y, x))
    print(len(positions))


def solvePartTwo():
    y, x = 0, 0
    for r, row in enumerate(grid):
        for c, cell in enumerate(row):
            if cell == "^":
                y, x = r, c
                break
        else:
            continue
        break

    def is_loop(grid, yPos, xPos):
        delta = (-1, 0)
        visited_states = set()

        while True:
            state = (yPos, xPos, delta)
            if state in visited_states:
                return True
            visited_states.add(state)
            nextY, nextX = yPos + delta[0], xPos + delta[1]

            if nextY < 0 or nextY >= len(grid) or nextX < 0 or nextX >= len(grid[0]):
                return False

            if grid[nextY][nextX] == "#":
                if delta == (-1, 0):
                    delta = (0, 1)
                elif delta == (0, 1):
                    delta = (1, 0)
                elif delta == (1, 0):
                    delta = (0, -1)
                elif delta == (0, -1):
                    delta = (-1, 0)
            else:

                yPos, xPos = nextY, nextX

    count = 0
    for cr in range(len(grid)):
        for cc in range(len(grid[0])):
            if grid[cr][cc] != ".":
                continue
            grid[cr][cc] = "#"
            if is_loop(grid, y, x):
                count += 1
            grid[cr][cc] = "."

    print(count)


solve()
solvePartTwo()
