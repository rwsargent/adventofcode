import sys
import pdb
def main():
    if (len(sys.argv) != 2):
        print("Specify module name")
        return -1
    
    name = sys.argv[1]
    mod = __import__(name, fromlist=[''])
    problem_input = mod.readinput()
    part_one_result = mod.execute_part_one(problem_input)
    part_two_result = mod.execute_part_two(problem_input)
    print(name)
    print("Part one: " + str(part_one_result))
    print("Part two: " + str(part_two_result))

if __name__ == "__main__":
    main()
    
