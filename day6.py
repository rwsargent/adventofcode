import re
import pdb
def read_input(filename):
    instructions = []
    regex = re.compile("([0-9]+)")
    with open(filename) as file:
        for line in file:
            x1, y1, x2, y2 = [int(x) for x in regex.findall(line)]
            action = get_nordic_action(line)
            instructions.append( ((x1, y1, x2, y2), action))
    return instructions
        
def execute_part_one(input):
    lights = set()
    for line in input:
        coordinates, action = line
        x1, y1, x2, y2 = coordinates
        for x in range(x1, x2+1):
            for y in range(y1, y2+1):
                action(lights, x, y)
    return len(lights)

def execute_part_two(input):
    lights = [[0 for i in range(1000)] for j in range(1000)]
    for line in input:
        coordinates, action = line
        x1, y1, x2, y2 = coordinates
        for x in range(x1, x2+1):
            for y in range(y1, y2+1):
                action(lights, x, y)
    brightness = 0
    for x in range(1000):
        for y in range(1000):
            brightness += lights[x][y]
    return brightness

def get_expected_results_map():
    return { 'test.txt' : (None, 1001996)}

def turn_on(lights, x, y):
    lights.add((x,y))

def turn_off(lights, x, y):
    light = (x,y)
    if light in lights:
        lights.remove((x,y))

def toggle(lights, x, y):
    light = (x,y)
    if light in lights:
        lights.remove(light)
    else:
        lights.add(light)
        
def nordic_on(lights, x, y):
    lights[x][y] += 1
def nordic_off(lights, x, y):
    if lights[x][y] > 0:
        lights[x][y] -= 1
def nordic_toggle(lights, x, y):
    lights[x][y] += 2

def get_action(line):
    if line.startswith("turn on"):
        return turn_on
    elif line.startswith("turn off"):
        return turn_off
    elif line.startswith("toggle"):
        return toggle

def get_nordic_action(line):
    if line.startswith("turn on"):
        return nordic_on
    elif line.startswith("turn off"):
        return nordic_off
    elif line.startswith("toggle"):
        return nordic_toggle
