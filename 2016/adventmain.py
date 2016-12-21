#!/usr/bin/python
import sys
import os
import pdb
import argparse
def run_tests(module, args):
    allPass = True
    for filename in os.listdir("tests/" + module.__name__):
      if(filename.startswith(".")): #skip 'hidden' files
        continue
      if args.test and args.test != filename:
        continue
      else:
        try:
          run_test(module, args, filename)
        except TestFailed as err:
          print(err.get_message())
          allPass = False
            
    return allPass

def run_test(module, args, filename):
  full_filename = "tests/" + module.__name__ + "/" + filename
  file_input = module.read_input(full_filename)
  try:
    expected_results = module.get_expected_results_map()[filename]
    if(expected_results == (None, None)):
      return 
  except KeyError:
    print("No expected value for " + filename)
    return
  
  part_one_test = module.execute_part_one(file_input) if args.skip != 1 or not expected_results[0] else None
  part_two_test = module.execute_part_two(file_input) if args.skip != 2 or not expected_results[2] else None
  
  if(part_one_test != expected_results[0] and args.skip != 1):
    maybe_print_fail_test_message(filename, 1, part_one_test, expected_results[0])
  if(part_two_test != expected_results[1] and args.skip != 2):
    maybe_print_fail_test_message(filename, 2, part_two_test, expected_results[1])

def maybe_print_fail_test_message(filename, part, result, expected):
    if(expected): #my hack for having a test file for only part one or two.
        message = "{0} part {1} test failed. Expected {2}, but got {3}".format(filename, part, expected, result)
        raise TestFailed(message)
                   
def parse_args():
    parser = argparse.ArgumentParser(description='Run and test Advent of Code puzzles!')
    parser.add_argument('module', metavar='mod', 
                        help='name of the module you wish to run')
    parser.add_argument('--skip', dest="skip", type=int, help='Skip the first puzzle')
    parser.add_argument('--test', dest="test", type=str, help='Run a specific test')

    return parser.parse_args()
def main():
    if (len(sys.argv) < 2):
        print("Specify module name")
        return -1
    args = parse_args()
    name = args.module
    mod = __import__(name, fromlist=[''])
    if not run_tests(mod, args):
      return
    print "All Tests passed!"
    problem_input = mod.read_input("input/" + mod.__name__ + "_input.txt")
    if(args.skip != 1):
        part_one_result = mod.execute_part_one(problem_input)
        print("Part one: " + str(part_one_result))
    if(args.skip != 2):
        part_two_result = mod.execute_part_two(problem_input)
        print("Part two: " + str(part_two_result))

class TestFailed(Exception):
    def __init__(self, message):
        self.message = message;

    def get_message(self):
        return self.message

if __name__ == "__main__":
    main()
    
