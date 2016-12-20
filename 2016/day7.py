import re
import pdb
regex_string = r'((\w)(?!\2)(\w)\3\2)(?![^\[]*\])'
inner_regex = re.compile(r'((\w)(?!\2)(\w)\3\2)(?=[^\[]*\])')
abba_regex = re.compile(regex_string)

def read_input(filename):
  with open(filename) as file:
    return [line for line in file]
        
def execute_part_one(ipv7s):
  count = 0
  for address in ipv7s:
    match = abba_regex.search(address)
    if match:
      if not inner_regex.search(address):
        count += 1
        
  return count

def execute_part_two(ipv7s):
  inside = re.compile(r'((\w)(?!\2)(\w)\2)(?![^\[]*\])(.*?\3\2\3)(?=[^\[]*\])') #checks aba[bab]
  outside = re.compile(r'((\w)(?!\2)(\w)\2)(?=[^\[]*\])(.*?\3\2\3)(?![^\[]*\])') #checks [bab]aba
  count = 0
  for address in ipv7s:
    if inside.search(address) or outside.search(address):
      count += 1
  return count

def get_expected_results_map():
    return { 'test.txt' : (3, None), 'test2.txt' : (None, 3)}
