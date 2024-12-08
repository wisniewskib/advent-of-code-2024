grid = [line.strip() for line in open("./inputs/day8.txt")]

rows = len(grid)
cols = len(grid[0])

antennas = {}


def isInGrid(r, c):
    return 0 <= r < rows and 0 <= c < cols


for r, row in enumerate(grid):
    for c, col in enumerate(row):
        if not col == ".":
            if col not in antennas:
                antennas[col] = []
            antennas[col].append((r, c))

antinodes = set()
for array in antennas.values():
    for i in range(len(array)):
        for j in range(i + 1, len(array)):
            r1, c1 = array[i]
            r2, c2 = array[j]
            antinodes.add((2 * r1 - r2, 2 * c1 - c2))
            antinodes.add((2 * r2 - r1, 2 * c2 - c1))


antinodesPartTwo = set()
for array in antennas.values():
    for i in range(len(array)):
        for j in range(len(array)):
            if i == j:
                continue
            r1, c1 = array[i]
            r2, c2 = array[j]
            deltaR = r2 - r1
            deltaC = c2 - c1

            r, c = r1, c1
            while isInGrid(r, c):
                antinodesPartTwo.add((r, c))
                r += deltaR
                c += deltaC

print(len([0 for r, c, in antinodes if isInGrid(r, c)]))
print(len([0 for r, c, in antinodesPartTwo if isInGrid(r, c)]))
