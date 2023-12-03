import re

file_path = "/Users/andilar/Documents/GitHub/AdventOfCode/2023/01/data.txt"
coordinates = [[0,0]]
final_coord = 0

replacements = {
    "one": "one1one",
    "two": "two2two",
    "three": "three3three",
    "four": "four4four",
    "five": "five5five",
    "six": "six6six",
    "seven": "seven7seven",
    "eight": "eight8eight",
    "nine": "nine9nine"
}


def replace_string_with_fitting_number(str,sub,rpl):
    substring_to_replace = sub
    replacement_string = rpl

    # Using replace() method with case-insensitive comparison
    result_string = str.lower().replace(substring_to_replace.lower(), replacement_string)

    return result_string


with open(file_path, "r") as file:
    for line in file:

        # Iterate through the replacements and apply them
        for target, replacement in replacements.items():
            line = replace_string_with_fitting_number(line, target, replacement)

        # never been good in regex though..
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