#!/usr/bin/python
import sys
import os
import pdb
import argparse
def run_tests(module, args):
    expected_results_map = module.get_expected_results_map()
    name = module.__name__
    for filename in os.listdir("tests/" + name):
        if(filename.startswith(".")): #skip 'hidden' files
            continue
        full_filename = "tests/" + name + "/" + filename
        file_input = module.read_input(full_filename)
        part_one_test = module.execute_part_one(file_input) if args.skip != 1 else None
        part_two_test = module.execute_part_two(file_input) if args.skip != 2 else None
        try:
            expected_results = expected_results_map[filename]
            if(part_one_test != expected_results[0] and args.skip != 1):
                    maybe_print_fail_test_message(filename, 1, part_one_test, expected_results[0])
            if(part_two_test != expected_results[1] and args.skip != 2):
                maybe_print_fail_test_message(filename, 2, part_two_test, expected_results[1])
        except KeyError:
            print("No expected value for " + filename)
            
    print ("All tests pass")
    return True

def maybe_print_fail_test_message(filename, part, result, expected):
    if(expected): #my hack for having a test file for only part one or two.
        message = "{0} part {1} test failed. Expected {2}, but got {3}".format(filename, part, expected, result)
        raise TestFailed(message)
                   
def parse_args():
    parser = argparse.ArgumentParser(description='Run and test Advent of Code puzzles!')
    parser.add_argument('module', metavar='mod', 
                        help='name of the module you wish to run')
    parser.add_argument('--skip', dest="skip", type=int, help='Skip the first puzzle')

    return parser.parse_args()
def main():
    if (len(sys.argv) < 2):
        print("Specify module name")
        return -1
    args = parse_args()
    name = args.module
    mod = __import__(name, fromlist=[''])
    try:
        run_tests(mod, args)
    except TestFailed as err:
        print(err.get_message())
        return

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
    
