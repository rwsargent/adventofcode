def read_input(filename):
  print(filename)
  with open(filename) as file:
    data = file.read().split(", ")
    return [(datum[0], int(datum[1:])) for datum in data]
      
    
def execute_part_one(input):
  blockmap = [0, 0, 0, 0]
  direction = 0
  for instruction in input:
    turn, amount = instruction
    direction = ((direction + 1) if turn == 'R' else (direction + 3)) % 4
    blockmap[direction] += amount

  NS = abs(blockmap[0] - blockmap[2])
  EW = abs(blockmap[1] - blockmap[3])
  return NS + EW

def execute_part_two(input):
  coords = [0, 0]
  visited = set()
  blockmap = [0, 0, 0, 0]
  direction = 0

  count = 0
  for instruction in input:
    count += 1
    print(count)
    turn, amount = instruction
    direction = ((direction + 1) if turn == 'R' else (direction + 3)) % 4
    start = (coords[0], coords[1])
    
    if direction == 0:
      coords[1] += amount
    elif direction == 1:
      coords[0] += amount
    elif direction == 2:
      coords[1] -= amount
    elif direction == 3:
      coords[0] -= amount
      
    traveled = (start, (coords[0], coords[1]))
    print(traveled)
    for line in visited:
      intersection = line_intersection(line, traveled)
      if intersection != None:
        if intersection != start:
          print("Intersection: " + str(intersection) + "between " + str(line), str(traveled))
          return abs(intersection[0]) + abs(intersection[1])
    visited.add(traveled)
      
  return None

def get_expected_results_map():
    return { 'test.txt' : (5, None), 'test2.txt' : (2, None), 'test3.text' : (12, None), '2.txt' : (None, 4)}


def line_intersection(line1, line2):
  xdiff = (line1[0][0] - line1[1][0], line2[0][0] - line2[1][0])
  ydiff = (line1[0][1] - line1[1][1], line2[0][1] - line2[1][1]) #Typo was here

  def det(a, b):
    return a[0] * b[1] - a[1] * b[0]

  div = det(xdiff, ydiff)
  if div == 0:
    return None

  d = (det(*line1), det(*line2))
  x = det(d, xdiff) / div
  y = det(d, ydiff) / div

  #test2
  if (line2[0][0] == line2[1][0]):  #same x:
    if not (y >= min(line2[0][1], line2[1][1]) and y <= max(line2[0][1], line2[1][1])):
      return None
  else: #same y
    if not (x >= min(line2[0][0], line2[1][0]) and x <= max(line2[0][0], line2[1][0])):
      return None

  if (line1[0][0] == line1[1][0]):  #same x:
    if not (y >= min(line1[0][1], line1[1][1]) and y <= max(line1[0][1], line1[1][1])):
      return None
  else: #same y
    if not (x >= min(line1[0][0], line1[1][0]) and x <= max(line1[0][0], line1[1][0])):
      return None
    
  return x, y
    

    
