import re

functions = { 'inc' : lambda reg,arg,line: (reg+1, line+1),
              'jio' : lambda reg,arg,line: (reg, eval(str(line)+arg) if reg==1 else line+1),
              'tpl' : lambda reg,arg,line: (reg*3, line+1),
              'hlf' : lambda reg,arg,line: (reg/2, line+1),
              'jie' : lambda reg,arg,line: (reg, eval(str(line)+arg) if reg%2==0 else line+1),
              'jmp' : lambda reg,arg,line: (reg, eval(str(line)+arg))}
def read_input(filename):
    pattern = re.compile(r"([a-z]{3}) ([ab])?(?:.*([+-]\d+))?")
    instructions = []
    with open(filename) as file:
        for line in file:
            line = line.strip()
            match = pattern.match(line)
            instructions.append((match.group(1), match.group(2), match.group(3)))
    return instructions
        
def execute_part_one(input):
    line = 0
    registers = {'a' : 1, 'b': 0, None : None}
    try:
        while(True):
            instruction, reg_key, arg = input[line]
            reg, line = functions[instruction](registers[reg_key], arg,line)
            registers[reg_key] = reg
    except IndexError:
        return registers['b']

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}
