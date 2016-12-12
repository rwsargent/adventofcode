from collections import namedtuple

def read_input(filename):
  with open(filename) as file:
    return [line for line in file]


def execute_part_one(button_string):
  keypad = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]]
  key = namedtuple("Key", "row col")
  key.row = 1
  key.col = 1
  code = ""
  for string in button_string:
    for command in string:
      if command == "U":
        moveUp(key)
      elif command == "D":
        moveDown(key)
      elif command == "L":
        moveLeft(key)
      elif command == "R":
        moveRight(key)
    code += keypad[key.row][key.col]
    
  return code

def execute_part_two(button_string):
  keypad = [[None, None, "1", None, None],
            [None, "2", "3", "4", None],
            ["5", "6",  "7", "8", "9"],
            [None, "A", "B", "C", None],
            [None, None, "D", None, None]]
  key = namedtuple("Key", "row col")
  key.row = 2
  key.col = 0
  code = ""
  for string in button_string:
    for command in string:
      if command == "U":
        moveUp2(key, keypad)
      elif command == "D":
        moveDown2(key, keypad)
      elif command == "L":
        moveLeft2(key, keypad)
      elif command == "R":
        moveRight2(key, keypad)
    print(str(key.row) + ", " + str(key.col))
    code += keypad[key.row][key.col]
    
  return code

def get_expected_results_map():
    return { 'test.txt' : ("1985", "5DB3")}


def moveUp2(key, keypad):
  if key.row != 0 and keypad[key.row -1][key.col] != None:
    key.row = key.row - 1
def moveDown2(key, keypad):
  if key.row != 4 and keypad[key.row + 1][key.col] != None:
    key.row = key.row + 1
def moveLeft2(key, keypad):
  if key.col != 0 and keypad[key.row][key.col -1] != None:
    key.col = key.col - 1
def moveRight2(key, keypad):
  if key.col != 4 and keypad[key.row][key.col + 1] != None:
    key.col = key.col + 1
  
def moveUp(key):
  if(key.row != 0):
    key.row = key.row -1

def moveDown(key):
  if(key.row != 2):
    key.row = key.row+1

def moveLeft(key):
  if key.col != 0:
    key.col = key.col - 1

def moveRight(key):
  if key.col != 2:
    key.col = key.col + 1
