#!/usr/bin/python
import sys
import os
import pdb

def run_tests(module):
    expected_results_map = module.get_expected_results_map()
    name = module.__name__
    for filename in os.listdir("tests/" + name):
        full_filename = "tests/" + name + "/" + filename
        file_input = module.read_input(full_filename)
        part_one_test = module.execute_part_one(file_input)
        part_two_test = module.execute_part_two(file_input)
        try:
            expected_results = expected_results_map[filename]
            if(part_one_test != expected_results[0]):
                print(filename + " part 1 test failed.")
                return False
                if(part_two_test != expected_results[1]):
                    if(expected_results[1]): #if we don't expect anything
                       print(filename + " part 2 test failed.")
                       return False
        except KeyError:
            print("No expected value for " + filename)
            
    print ("All tests pass")
    return True

                

def main():
    if (len(sys.argv) != 2):
        print("Specify module name")
        return -1
    
    name = sys.argv[1]
    mod = __import__(name, fromlist=[''])
    if(not run_tests(mod)):
        return
    problem_input = mod.read_input("input/" + mod.__name__ + "_input.txt")
    part_one_result = mod.execute_part_one(problem_input)
    part_two_result = mod.execute_part_two(problem_input)
    print(name)
    print("Part one: " + str(part_one_result))
    print("Part two: " + str(part_two_result))

if __name__ == "__main__":
    main()
    
