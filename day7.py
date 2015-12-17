import re, pdb
def read_input(filename):
    wires = {}
    gates = set()
    #((\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT)?)(?(1)\s)(\w+) -> ([a-z]+) -- the real one
    regex = re.compile(r'((\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT)?)(?(1)\s)(\w+) -> ([a-z]+)')
    with open(filename) as file:
        for line in file:
            match = regex.match(line)
            if(not match):
                print(line)
                raise Exception()
            gate = get_gate_of(match.group(3))
            wire_one = get_wire(match.group(2), wires)
            wire_two = get_wire(match.group(4), wires)
            wire_out = get_wire(match.group(5), wires)
            
            if (match.group(5) == "NOT" or match.group(5) == None):
                gate.inputs = [wire_two, wire_two]
            else:
                gate.inputs =[wire_one, wire_two]
            gate.output = wire_out
            wire_one.inputs.append(gate)
            wire_two.inputs.append(gate)
            gates.add(gate)
    return gates, wires
        
def execute_part_one(input):
    for gate in gates:
        gate.go()

def execute_part_two(input):
    return None

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

def get_gate_of(bool_type):
    if bool_type == "NOT":
        return Gate(lambda a,b: ~a)
    elif bool_type == "AND":
        return Gate(lambda a,b:a&b)
    elif bool_type == "OR":
        return Gate(lambda a,b:a|b)
    elif bool_type == "RSHIFT":
        return Gate(lambda a,b: a>>b)
    elif bool_type == "LSHIFT":
        return Gate(lambda a,b: a<<b)
    else:
        return Gate(lambda a, b: a)

def get_wire(key, map):
    if not key: #if the key is None, return a simple wire
        return Wire(signal=True)
    try: #if the wire is just a value, don't map it
        value = int(key)
        return Wire(value, signal=True)
    except:
        if key in map: #return a wire that's already been constructed
            return map[key]
        else:
            wire = Wire() #make a new wire and map it
            map[key] = wire
            return wire
        
class Wire:
    def __init__(self, value=0, inputs=[],signal=False):
        self.value = value
        self.inputs = inputs
        self.signal = signal

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
        return hash(self.inputs[0]) + hash(self.inputs[0]) * hash(self.output)
    
    def go(self):
        a, b = self.inputs
        if(a.signal and b.signal):
            self.output.value = (0xffff & logic(a,b))
            slef.output.signal = True
            for gate in self.output.inputs:
                gate.go()
'''
matches
group 1: lhs without last gate.
group 2: left arg
group 3: gate logic
group 4: right arg
group 5: output
'''
