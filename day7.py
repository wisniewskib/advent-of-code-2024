lines = open("./inputs/day7.txt").read().split("\n")


def check_combination(nums, target, concatenate=False):
    n = len(nums)

    def backtrack(index, current):
        if index == n:
            if current == target:
                return True
            return False

        add_result = backtrack(index + 1, current + nums[index])
        if add_result:
            return add_result

        mul_result = backtrack(index + 1, current * nums[index])
        if mul_result:
            return mul_result

        if concatenate:
            concatenate_result = backtrack(index + 1, int(f"{current}{nums[index]}"))
            if concatenate_result:
                return concatenate_result

        return False

    return backtrack(1, nums[0])


result = 0

for line in lines:
    target, numbers = line.split(":")
    numbers = list(map(int, numbers.strip().split(" ")))
    if check_combination(numbers, int(target)):
        result += int(target)

print(f"Part one: {result}")


resultPartTwo = 0

for line in lines:
    target, numbers = line.split(":")
    numbers = list(map(int, numbers.strip().split(" ")))
    if check_combination(numbers, int(target), True):
        resultPartTwo += int(target)

print(f"Part two: {resultPartTwo}")
