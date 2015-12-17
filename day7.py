import re, pdb
def read_input(filename):
    wires = {}
    gates = list()
    regex = re.compile(r'(([a-z0-9]+)?(?(2) )(AND|OR|LSHIFT|RSHIFT|NOT)?) ?(\w+) -> ([a-z]+)')
    with open(filename) as file:
        for line in file:
            match = regex.match(line)
            if(not match):
                print(line)
                raise Exception()
            gate = get_gate_of(match.group(3))
            wire_one = get_wire(match.group(2), wires, gate)
            wire_two = get_wire(match.group(4), wires, gate)
            wire_out = get_wire(match.group(5), wires)
            
            if (match.group(3) == "NOT" or match.group(3) == None):
                gate.inputs = [wire_two, wire_one]
            else:
                gate.inputs =[wire_one, wire_two]
            gate.output = wire_out
            gates.append(gate)
    return gates, wires
        
def execute_part_one(input):
    gates, wires = input
    counter = 0
    for gate in gates:
        counter += 1
        gate.go()
    return wires['a'].value

def execute_part_two(input):
    gates, wires = input
    for gate in gates:
        gate.go()
    a_val = wires['a'].value
    for key in wires:
        wire = wires[key]
        wire.signal = False
        wire.value = 0
    wires['b'].value = a_val
    wires['b'].signal = True
    for gate in gates:
        gate.go()
    pdb.set_trace()
    return wires['a'].value

def get_expected_results_map():
    return { 'test.txt' : (None, None), 'mytest.txt' : (None, None), 'adventtest.txt' : (None, None)}

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
        return Gate(lambda a, b: a.value)

def get_wire(key, map, gate=None):
    if not key: #if the key is None, return a simple wire
       return Wire("blank", signal=True)
    try: #if the wire is just a value, don't map it
        value = int(key)
        return Wire(key, value, True)
    except:
        if key in map: #return a wire that's already been constructed
            if(gate):
                map[key].inputs.append(gate)
            return map[key]
        else:
            wire = Wire(key) #make a new wire and map it
            if(gate):
                wire.inputs.append(gate)
            map[key] = wire
            return wire
        
class Wire:
    def __init__(self, name, value=0,signal=False):
        self.name = name
        self.value = value
        self.inputs = []
        self.signal = signal
    def __str__(self):
        return self.name
    def __and__(self, other):
        return self.value & other.value
    def __lshift__(self, other):
        return self.value << other.value
    def __rshift__(self, other):
        return self.value >> other.value
    def __or__(self, other):
        return self.value | other.value
    def __invert__(self):
        return ~self.value

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
        if(a.signal and b.signal and not self.output.signal):
            self.output.value = (self.logic(a,b) & 0xffff )
            self.output.signal = True
#            print(a.name + " and " + b.name + " result in: " + str(self.output.value))
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
