import re
def read_input(filename):
    wires = {}, gates = set()
    # ((\d+)|((\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT )))(\w+) -> ([a-z]+)
    regex = re.compile(r'((\d+)|((\d+|\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT) (\w+))) -> ([a-z]+)')
    with open(filename) as file:
        for line in file:
            match = regex.match(line)
            gate = get_gate_of(match.group(5))
            wire_one,wire_two = Wire()
            wires[match.group(4)] = wire_one
            wires[match.group(6)] = wire_two
            if (match.group(5) == "NOT"):
                gate.inputs = [
                
    return wires, regex
        
def execute_part_one(input):
    return None

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

def get_gate_of(boolean, inputs, outputs):
    if boolean == "NOT":
        return Gate(bit_not)
    elif boolean == "AND":
        return Gate(bit_and)
    elif boolean == "OR":
        return Gate(bit_or)
    elif boolean == "RSHIFT":
        return Gate(bit_rshift)
    elif boolean == "LSHIFT":
        return Gate(bit_lshift)
            

def bit_not(value):
    return ~value
def bit_or(a, b):
    return a | b
def bit_rshift(a, b):
    return a << b
def bit_lshift(a, b):
    return a << b
def bit_and(a, b):
    return a & b
class Wire:
    def __init__(self, value=0, input=None):
        self.value = value
        self.input = input

class Gate:
    def __init__(self,logic):
        self.logic = logic
        self.inputs = []
        self.output = None
    def __eq__(self, other):
        if not isinstance(other, Gate):
            return False
        return self.inputs == other.inputs and self.output == other.output
    def __hash__(self):
        return hash(self.inputs) * hash(self.outputs)
        
'''
matches
group 1: arguments
group 2: number argument if only number
group 3: arguments
group 4: left arg
group 5: gate logic
group 6: r arg
group 7: out gate
'''
