import re
import pdb
def read_input(filename):
    lines = []
    with open(filename) as file:
        for line in file:
            lines.append(line.strip())
    return "".join(lines)
        
def execute_part_one(input, goal):
    pattern = re.compile(r'(-?\d+)')
    return reduce(lambda x, y: int(x) + int(y), pattern.findall(input), 0)

def execute_part_two(input, goal):
    import json
    JSON = json.loads(input)
    return sumObj(JSON, 0)


def sumObj(obj, acc):
    obj_sum = 0
    valid = True
    for item in obj:
        item = obj[item] if type(obj) == dict else item
        if type(item) == int:
            obj_sum += int(item)
        elif type(item) == list or type(item) == dict:
            obj_sum += sumObj(item, acc)
        elif type(item) == unicode:
            if item == 'red':
                if not type(obj) == list:
                    valid = False
    if(valid):
        acc += obj_sum
    return acc
def get_expected_results_map():
    return { 'test.txt' : (12, 6)}
def get_test_goal():
    return 0
def get_problem_goal():
    return 0
