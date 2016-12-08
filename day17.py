import itertools
def read_input(filename):
    size_array = []
    with open(filename) as file:
        for line in file:
            size_array.append(int(line.strip()))
    return size_array
        
def execute_part_one(input):
    ps =  [x for x in powerset(input)]
    counter =0
    for pset in ps:
        acc = sum(pset)
        if (acc == 150):
            counter += 1
    return counter

def execute_part_two(input):
    ps =  [x for x in powerset(input)]
    size_map = [0 for x in range(len(input)+1)]
    for pset in ps:
        acc = sum(pset)
        if (acc == 150):
            size_map[len(pset)] += 1
    #return the first non-zero number
    for x in size_map:
        if x != 0:
            return x

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

def recurs_fill(sizes, i, total):
    if (i >= len(sizes) or total < 0):
        return 0
    if (total-sizes[i] == 0):
        return 1
    return recurs_fill(sizes, i+1, total-sizes[i]) + recurs_fill(sizes, i+1, total)
    
def powerset(seq):
    """
    Returns all the subsets of this set. This is a generator.
    """
    if len(seq) <= 1:
        yield seq
        yield []
    else:
        for item in powerset(seq[1:]):
            yield [seq[0]]+item
            yield item
