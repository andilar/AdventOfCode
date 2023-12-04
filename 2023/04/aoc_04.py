import re

file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/04/data.txt"

points = 0

with open(file_path, 'r') as file:
    for line in file:
        # separator @40
        # first number @10 + 11

        before_separator, after_separator = line.split("|")

        numbers_before_separator = [int(num) for num in before_separator.split() if num.isdigit()]

        numbers_after_separator = [int(num) for num in after_separator.split() if num.isdigit()]

        for num in numbers_before_separator:
            if num in numbers_after_separator:
                print(f"Number {num} is found before and after the separator.")
                points += 1
                
    print (points)
