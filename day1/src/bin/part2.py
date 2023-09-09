input = input()

n = 0
count = 0
for c in input:
    count += 1
    if c == "(":
        n += 1
    if c == ")":
        n -= 1
    if n == -1:
        break

print(count - 1)
