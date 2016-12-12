ITERATIONS = 40
def read_input(filename):
    with open(filename) as file:
        for line in file:
            return line.strip()
        
def execute_part_one(input):
    
    return None

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

def look_and_say(string):
    prev = ' '
    length = 1
    output = ""
    for char in string:
        if char == prev:
            length += 1
        else:
            output += str(length) + str(prev)
            prev = char
            length = 1
    return output

