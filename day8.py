import re
def read_input(filename):
    string_size = 0
    memory_size = 0
    hexi = re.compile(r"(\\x[a-f0-9]{2})")
    escape = re.compile(r"((?<=[^\\]\\).)")
    with open(filename) as file:
        for line in file:
            line = line.strip()
            string_size += len(line)
            line = hexi.sub("*", line)
            line = escape.sub("", line)[1:-1]
            memory_size += len(line)
    return string_size, memory_size
        
def execute_part_one(input):
    string, mem = input
    return string - mem

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (12, None)}
