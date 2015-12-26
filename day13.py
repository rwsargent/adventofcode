import re
import pdb
import itertools
def read_input(filename):
    pattern = re.compile(r'(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)')
    guests = {}
    with open(filename) as file:
        for line in file:
            sitter, diff, amount, sittee = pattern.findall(line.strip())[0]
            guest = get_guest(sitter, guests)
            guest[sittee] = eval(('+' if diff == 'gain' else '-') + amount)
    return guests
        
def execute_part_one(input):
    perms = itertools.permutations(input.keys())
    max_score = -1
    for perm in perms:
        score = 0
        for i in range(len(perm)):
            lg_name = perm[i]
            rg_name = perm[(i+1)%len(perm)]
            leftguest = input[lg_name]
            rightguest = input[rg_name]
            score += leftguest[rg_name] + rightguest[lg_name]
            max_score = max(max_score, score)
    return max_score

def execute_part_two(input):
    #add self.
    guests = input.keys()
    me = {}
    for guest in guests:
        input[guest]['me'] = 0
        me[guest] = 0
    input['me'] = me
    perms = itertools.permutations(input.keys())
    max_score = -1
    for perm in perms:
        score = 0
        for i in range(len(perm)):
            lg_name = perm[i]
            rg_name = perm[(i+1)%len(perm)]
            leftguest = input[lg_name]
            rightguest = input[rg_name]
            score += leftguest[rg_name] + rightguest[lg_name]
            max_score = max(max_score, score)
    return max_score

def get_expected_results_map():
    return { 'test.txt' : (330, None)}

def get_guest(guest, guests):
    if guest not in guests:
        guests[guest] = {}
    return guests[guest]
