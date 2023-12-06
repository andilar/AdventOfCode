#
import re
from math import sqrt, floor, ceil

file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/06/data.txt"


race_time = []
time_to_beat = []
hack = 0

with open(file_path, 'r') as file:
    for line in file:
        numbers = re.findall(r'\d+', line)
        if 0 ==hack:
            for num in numbers:
                race_time.append(int(num))
            hack = 1
        else:
            for num in numbers:
                time_to_beat.append(int(num))

def calc_race(rt, ttb):
    result = 0
    for x in range(rt+1):
        dx = x*(rt-x)
        if dx >= ttb:
            result += 1
    return result
    
result = 1
for i in range(len(race_time)):
    result *= calc_race(race_time[i], time_to_beat[i])

print(race_time)
print(time_to_beat)    
print(result)