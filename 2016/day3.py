def read_input(filename):
    with open(filename) as file:
      return [ map(lambda x: int(x), filter(lambda x: len(x) > 0, line.split(" "))) for line in file]
    
def execute_part_one(triangles):
  count = 0
  for triangle in triangles:
    sort = sorted(triangle)
    if(sort[0] + sort[1] > sort[2]):
      count += 1
      
  return count

def execute_part_two(triangles):
  
  flattened = [ triangles for s in a] 
  return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}
