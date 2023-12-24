sum = 1
diff = 15439
steps = 26501365/131 + 1
print(f"{steps} steps")

for i in range(int(steps) + 1):
    sum += diff
    diff += 30442
    if i in [1,2,3,4,5,6,202300]:
        print(f"{i-1}: {sum} (current diff: {diff})")

print(sum)

# solution below

fn(202301)
fn = lambda x: 15221*x*x - 15104*x + 3765