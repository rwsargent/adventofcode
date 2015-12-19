import re
def read_input(filename):
    lines = []
    with open(filename) as file:
        for line in file:
            lines.append(line.strip())
    return lines

def execute_part_one(input):
    string_size = 0
    memory_size = 0
    backslashes = re.compile(r'\\\\')
    quotes = re.compile(r'(\\")')
    hexi = re.compile(r"(?<!\\)(\\x[a-f0-9]{2})")
    for line in input:
        string_size += len(line)
        line = backslashes.sub('*', line)[1:-1]
        line = quotes.sub('*', line)
        line = hexi.sub("*", line)
        memory_size += len(line)
    return string_size - memory_size

def execute_part_two(input):
    acc = 0
    for line in input:
        acc += 2+line.count('\\')+line.count('"')
    return acc
    
                 
               
def get_expected_results_map():
    return { 'test.txt' : (12, 19)}
