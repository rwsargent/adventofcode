import re
def read_input(filename):
  with open(filename) as file:
    return "".join(file.readlines())
  
def execute_part_one(compressed):
  marker_pattern = re.compile(r'\((\d+)x(\d+)\)') #Matches (\d+x\d), eg (4x1)(90x1)
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

def execute_part_two(input):
  return None

def get_expected_results_map():
  return { 'test.txt' : (None, None),
           'a.txt' : (6, None),
           'abc.txt' : (7,None),
           'abcdefg.txt' : (11,None),
           'advent.txt' : (6,None),
           'xabcy.txt' : (18,20),
           'xyz.txt' : (9,9),
           'SEVEN.txt': (None, 445),
           'biga.txt' : (None, 241920)
  }

def decompress_run(repeat_count, run, marker_pattern):
  
  pass
