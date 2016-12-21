from collections import namedtuple
import pdb
import pprint
Command = namedtuple("Command", "doWork x y")
def read_input(filename):
  commands = []
  with open(filename) as file:
    return file.readlines()
              
def execute_part_one(input):
  grid = [["." for col in range(50)] for row in range(6)]
  for line in input:
    if line.startswith("rect"):
      width, height = line[5:].split("x")
      drawRect(grid, int(width), int(height))
    elif line.startswith("rotate row"):
      row, amount = line[len("rotate row y="):].split(" by " )
      rotate_row(grid, int(row), int(amount))
    elif line.startswith("rotate col"):
      col, amount = line[len("rotate column x="):].split(" by ")
      rotate_col(grid, int(col), int(amount))
  count = 0
  for row in grid:
    for col in row:
      if col == "#":
        count += 1
  printGrid(grid)
  return count

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (6, None)}


def drawRect(grid, width, height):
  for row in range(height):
    for col in range(width):
      grid[row][col] = "#"
  pass

def rotate_row(grid, row, amount):
  row_size = len(grid[row])
  grid[row] = [ grid[row][(i+amount)%row_size] for i in range(row_size)]

def rotate_col(grid, col, amount):
  pdb.set_trace()
  column = [row[col] for row in grid]
  
  #roate col
  new_col = [column[(i + amount) % len(column)] for i in range(len(column))]
  colIdx = 0
  for row in grid:
    row[col] = new_col[colIdx]
    colIdx += 1

def printGrid(grid):
  for line in grid:
    print("".join(line))
  
  
