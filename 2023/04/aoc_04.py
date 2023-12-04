
file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/04/data.txt"

points = 0

with open(file_path, 'r') as file:
    for line in file:
        temp_points=0

        before_separator, after_separator = line.split("|")
        numbers_before_separator = [int(num) for num in before_separator.split() if num.isdigit()]
        numbers_after_separator = [int(num) for num in after_separator.split() if num.isdigit()]

        for num in numbers_before_separator:
            if num in numbers_after_separator:
                print(f"Number {num} is found before and after the separator.")
                if temp_points == 0:
                    temp_points += 1
                else:
                    temp_points = temp_points * 2
        points += temp_points

    print (points)
