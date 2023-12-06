#
import re

file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/06/data.txt"


race_time = []
time_to_beat = []
hack = 0

with open(file_path, 'r') as file:
    for line in file:
        numbers = re.findall(r'\d+', line)
        if 0==hack:
            for num in numbers:
                race_time.append(num)
            hack = 1
        else:
            for num in numbers:
                time_to_beat.append(num)
            
print(race_time)
print(time_to_beat)