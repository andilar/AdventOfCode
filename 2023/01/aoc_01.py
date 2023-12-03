import re

file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/01/data.txt"
coordinates = [[0,0]]
final_coord = 0

with open(file_path, "r") as file:
    for line in file:
        # never been good in regex..
        numbers = re.findall(r'\d+', line)
        numbers = list(map(int, numbers))

        # hack for getting first and last digit
        first_num_str = str(numbers[0])
        first_digit = int(first_num_str[0])
        last_num_str = str(numbers[-1])
        last_digit = int(last_num_str[-1])

        row = [first_digit, last_digit]
        coordinates.append(row)

# build the numbers    
concatenated_numbers = [int(''.join(map(str, coord))) for coord in coordinates]

# add all numbers
for cn in concatenated_numbers:
    final_coord += cn

print (final_coord) 
