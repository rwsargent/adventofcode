import re
marker_pattern = re.compile(r'\((\d+)x(\d+)\)') #Matches (\d+x\d), eg (4x1)(90x1)

def read_input(filename):
  with open(filename) as file:
    return "".join(file.readlines())
  
def execute_part_one(compressed):
  '''
  Decompress the string! I search for every compression marker, and build up a decompression string.
  '''
  match = marker_pattern.search(compressed)
  data_region_idx = 0
  decompressed = ""
  while match != None:
    decompressed += compressed[data_region_idx:match.start()] #grab everything before the next marker

    #Decompression
    run_length = int(match.group(1))
    repeat_count = int(match.group(2))
    expanded_string = compressed[match.end():match.end()+run_length] * repeat_count
    decompressed += expanded_string #decompress the marker/data region
    
    data_region_idx = match.end()+run_length #reassign end of data region
    match = marker_pattern.search(compressed, data_region_idx) #find the next match
  decompressed += compressed[data_region_idx:]
  return len(decompressed.strip())

def execute_part_two(compressed):
  return decompress(compressed.strip())  

def get_expected_results_map():
  return { 'test.txt' : (None, None),
           'biga.txt' : (None, 241920)
  }

def decompress(compressed):
  '''
  Knowning that we just want the size, I don't build up a decompressed string. Instead,
  I recursively calculate the length of the string, but doing simple index arithmetic. 
  '''
  match = marker_pattern.search(compressed)
  data_region_idx = 0
  decompressed = 0
  while match != None:
    decompressed += match.start() - data_region_idx
    #Decompression
    run_length = int(match.group(1))
    repeat_count = int(match.group(2))
    decompressed += repeat_count * decompress(compressed[match.end():match.end()+run_length])
    
    data_region_idx = match.end()+run_length #reassign end of data region
    match = marker_pattern.search(compressed, data_region_idx) #find the next match
  decompressed += len(compressed) - data_region_idx
  return decompressed
