import re
abba_regex = re.compile(r'((\w)(?!\2)(\w)\3\2)(?![^{]*})')

def read_input(filename):
  with open(filename) as file:
    return [line for line in file]
        
def execute_part_one(ipv7s):
  for address in ipv7s:
    match = abba_regex.match(address)
    
  return None

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}
