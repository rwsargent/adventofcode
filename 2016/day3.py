def read_input(filename):
    with open(filename) as file:
      # This builds a list of ints
      return [ map(lambda x: int(x), #turn strings into ints
                   filter(lambda x: len(x) > 0, #only include elements that actually have them
                          line.split(" "))) for line in file] #split on space. This results in some elements as empty strings
    
def execute_part_one(triangles):
  count = 0
  for triangle in triangles:
    sort = sorted(triangle) #sort the lists
    if(sort[0] + sort[1] > sort[2]): #if the two smaller are bigger than biggest...its a triangle!
      count += 1
      
  return count

def execute_part_two(triangles):
  flattened = []
  trip = []
  for triangle in triangles: #first column
    trip.append(triangle[0])
    if len(trip) == 3:
      flattened.append(trip)
      trip = []
      
  for triangle in triangles: #middle column
    trip.append(triangle[1])
    if len(trip) == 3:
      flattened.append(trip)
      trip = []
      
  for triangle in triangles: #last column
    trip.append(triangle[2])
    if len(trip) == 3:
      flattened.append(trip)
      trip = []
    
  return execute_part_one(flattened)

def get_expected_results_map():
    return { 'test.txt' : (None, None)}
