import re
def read_input(filename):
    wires = {}, gates = set()
    #((\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT)?)(?(1)\s)(\w+) -> ([a-z]+) -- the real one
    regex = re.compile(r'((\w+ )?(AND|OR|LSHIFT|RSHIFT|NOT)?)(?(1)\s)(\w+) -> ([a-z]+)')
    with open(filename) as file:
        for line in file:
            match = regex.match(line)
            gate = get_gate_of(match.group(5) )
            wire_one = get_wire(match.group(4), wires)
            wire_two = get_wire(match.group(6), wires)
            wire_out = get_wire(match.group(7), wires)
            
            if (match.group(5) == "NOT" or match.group(5) == None):
                gate.inputs = [wire_two, Wire()]
            else():
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

def get_gate_of(boolean, inputs, outputs):
    if boolean == "NOT":
        return Gate(lambda a,b: ~a)
    elif boolean == "AND":
        return Gate(lambda a,b:a&b)
    elif boolean == "OR":
        return Gate(lambda a,b:a|b)
    elif boolean == "RSHIFT":
        return Gate(lambda a,b: a>>b)
    elif boolean == "LSHIFT":
        return Gate(lambda a,b: a<<b)
    else
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
        return hash(self.inputs) * hash(self.outputs)
    
    def go(self):
        a, b = self.inputs
        if(a.signal and b.signal):
            self.output.value = (0xffff & logic(a,b))
            slef.output.signal = True
            for(gate in self.output.inputs):
                gate.go()
                
    def get_input_values(self)
        
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
