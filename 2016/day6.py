from collections import Counter

def read_input(filename):
  with open(filename) as file:
    return [line for line in file]
  
def execute_part_one(lines):
  counters = [ Counter() for letter in lines[0]]
  for line in lines:
    for chIdx in range (len(line)):
      counters[chIdx].update(line[chIdx])

  return "".join([counter.most_common()[0][0] for counter in counters])

def execute_part_two(lines):
  counters = [ Counter() for letter in lines[0]]
  for line in lines:
    for chIdx in range (len(line)):
      counters[chIdx].update(line[chIdx])

  return "".join([counter.most_common()[-1][0] for counter in counters])


def get_expected_results_map():
    return { 'test.txt' : (None, None)}

