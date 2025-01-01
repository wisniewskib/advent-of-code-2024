input = open("./inputs/day9.txt").read().strip()

line = ""
id = 0

for i, char in enumerate(input):
    if i % 2 == 0:
        line += str(id) * int(char)
        id += 1
    else:
        line += "." * int(char)

arr = list(line)
left = 0
right = len(arr) - 1

while left < right:
    if arr[left] == ".":
        if arr[right] != ".":
            arr[left], arr[right] = arr[right], arr[left]
            left += 1
        right -= 1
    else:
        left += 1

print("".join(arr))
result = 0
for i, number in enumerate(arr):
    if number.isdigit():
        result += (i * int(number))

print(result)
