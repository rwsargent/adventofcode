def read_input(filename):
    size_array = []
    with open(filename) as file:
        for line in file:
            size_array.append(int(line.strip()))
    return size_array
        
def execute_part_one(input):
    return recurs_fill(input, 0, 150)

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

def recurs_fill(sizes, i, total):
    if (i >= len(sizes) or total < 0):
        return 0
    if (total == 0):
        return 1
    return recurs_fill(sizes, i+1, total-sizes[i]) + recurs_fill(sizes, i+1, total)
    
    
            
    
