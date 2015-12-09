import pdb
def read_input(filename):
    lines = []
    with open(filename) as file:
        for line in file:
            lines.append(line.strip())
    return lines
        
def execute_part_one(input):
    return check_list(input, is_nice)

def execute_part_two(input):
    return check_list(input, is_nice_part_two)

def get_expected_results_map():
     return { 'test1.txt' : (2, None), 
              'test2.txt' : (None, 2)
     }

def check_list(input, is_nice):
    nice_count = 0
    for word in input:
        if (is_nice(word)):
            nice_count += 1
    return nice_count
    
def is_nice(word):
    return has_three_vowels(word) and has_double_letter_and_no_bad_pairs(word)

def is_nice_part_two(word):
    return has_pair(word) and has_letter_splits(word)

def has_pair(word):
    pair_dict = {}
    for i in range(len(word)-1):
        pair = word[i:i+2]
        if pair in pair_dict:
            index = pair_dict[pair]
            if ((i - index) != 1): #they overlap!
                return True
            continue #don't update pair's index
        pair_dict[pair] = i

def has_letter_splits(word):
    for i in range(len(word)-2):
        triplet = word[i:i+3]
        if(triplet[0] == triplet[2]):
            return True
    return False
            

def has_three_vowels(word):
    vowels = "aeiou"
    count = 0
    for char in word:
        if char in vowels:
            count += 1
    return count >= 3

def has_double_letter_and_no_bad_pairs(word):
    naught_letters = ["ab", "cd", "pq", "xy"]
    doubles = False
    naughty_strings = False
    for chrIdx in range(len(word)-1):
        fst = word[chrIdx]
        snd = word[chrIdx +1]
        pair = fst + snd
        if(fst == snd):
            doubles = True
        if pair in naught_letters:
            naughty_strings = True
    return doubles and not naughty_strings
