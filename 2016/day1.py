def read_input(filename):
  '''
  Each line is parsed to be a tuple (D, A), where D is the direction I turn (right or left)
  and A is the amount I walk in that direction. 
  '''
  print(filename)
  with open(filename) as file:
    data = file.read().split(", ")
    return [(datum[0], int(datum[1:])) for datum in data]
      
    
def execute_part_one(input):
  '''
  The Ending position is equal to the number of steps I take North mins number of steps I take south,
  then the number of steps taken east minus number of steps west. If we start at (0, 0), we just need to
  calculate the number of steps I take in each direction, and resolve it on a Cartesian Co-ordiate plane.
  '''
  
  blockmap = [0, 0, 0, 0] #the counter for the cardinal directions in the following order: [N, E, S, W]
  direction = 0 #this will be an index into the block map. I start facing North, so index 0.
  
  for instruction in input:
    turn, amount = instruction
    #The way I layed out my direction map, if I turn right (i.e. N -> E) I increase my direction index by one.
    # If I turn left, (i.e. N -> W), I increase my direction by 3, and mod by 4 to keep me in the proper bounds.
    direction = ((direction + 1) if turn == 'R' else (direction + 3)) % 4 
    blockmap[direction] += amount #accumulate the number of steps I've taken in that direction

  NS = abs(blockmap[0] - blockmap[2]) #calculating |Y|
  EW = abs(blockmap[1] - blockmap[3]) #calcualting |X|
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
  '''stack overflow calculation for line intersection.
     Added code to be strictly line segment intersection.
  '''
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

  #Check bounds - enforce linesegment intersection.
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
    

    
