import os
import re


file_path = "data.txt"
coordinates = []


with open(file_path, "r") as file:
    for line in file:
        for lin in line:
            if lin.isdigit():
                coordinates.append(lin)
                

