#!/usr/bin/python
import sys
import os
import pdb

def run_tests(module):
    expected_results_map = module.get_expected_results_map()
    name = module.__name__
    for filename in os.listdir("tests/" + name):
        if(filename.startswith(".")): #skip 'hidden' files
            continue
        full_filename = "tests/" + name + "/" + filename
        file_input = module.read_input(full_filename)
        part_one_test = module.execute_part_one(file_input)
        part_two_test = module.execute_part_two(file_input)
        try:
            expected_results = expected_results_map[filename]
            if(part_one_test != expected_results[0]):
                maybe_print_fail_test_message(filename, 1, part_one_test, expected_results[0])
            if(part_two_test != expected_results[1]):
                maybe_print_fail_test_message(filename, 2, part_two_test, expected_results[1])
        except KeyError:
            print("No expected value for " + filename)
            
    print ("All tests pass")
    return True

def maybe_print_fail_test_message(filename, part, result, expected):
    if(expected): #my hack for having a test file for only part one or two.
        message = "{0} part {1} test failed. Expected {2}, but got {3}".format(filename, part, expected, result)
        raise TestFailed(message)
                   

def main():
    if (len(sys.argv) != 2):
        print("Specify module name")
        return -1
    
    name = sys.argv[1]
    mod = __import__(name, fromlist=[''])
    try:
        run_tests(mod)
    except TestFailed as err:
        print(err.get_message())
        return

    problem_input = mod.read_input("input/" + mod.__name__ + "_input.txt")
    part_one_result = mod.execute_part_one(problem_input)
    part_two_result = mod.execute_part_two(problem_input)
    print(name)
    print("Part one: " + str(part_one_result))
    print("Part two: " + str(part_two_result))

class TestFailed(Exception):
    def __init__(self, message):
        self.message = message;

    def get_message(self):
        return self.message

if __name__ == "__main__":
    main()
    
