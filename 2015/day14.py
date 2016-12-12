import re
import pdb
TIME = 2
SPED = 1
REST = 3
DIST = 0
POIN = 4
def read_input(filename):
    pattern = re.compile(r'([a-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.', re.I)
    deers = []
    with open(filename) as file:
        for line in file:
            deer, speed, time, rest = pattern.findall(line.strip())[0]
            deers.append((int(speed), int(time), int(rest)))
    return deers
        
def execute_part_one(input, goal):
    distances = [[0, deer[0], deer[1], deer[2]] for deer in input]
    for sec in range(goal+1):
        for i, deer in enumerate(distances):
            if (deer[TIME] > 0):
                deer[DIST] += deer[SPED]
                deer[TIME] -= 1
            elif (deer[REST] > 0):
                deer[REST] -= 1
            elif (deer[TIME] == 0 and deer[REST] == 0):
                deer[TIME], deer[REST] = input[i][1]-1, input[i][2]
                deer[DIST] += deer[SPED]
            else:
                print("shouldn't see this")
    dist = reduce(max, map(lambda x: x[0], distances))
    return dist

def execute_part_two(input, goal):
    distances = [[0, deer[0], deer[1], deer[2], 0] for deer in input]
    for sec in range(goal+1):
        winning_distance = 0
        for i, deer in enumerate(distances):
            if (deer[TIME] > 0):
                deer[DIST] += deer[SPED]
                deer[TIME] -= 1
            elif (deer[REST] > 0):
                deer[REST] -= 1
            elif (deer[TIME] == 0 and deer[REST] == 0):
                deer[TIME], deer[REST] = input[i][1]-1, input[i][2]
                deer[DIST] += deer[SPED]
            else:
                print("shouldn't see this")
            winning_distance = max(deer[DIST], winning_distance)
        for deer in filter(lambda x: x[DIST] == winning_distance, distances):
            deer[POIN] += 1
    points = reduce(max, map(lambda x: x[POIN], distances))
    return points
def get_expected_results_map():
    return { 'test.txt' : (1120, 689)}
def get_test_goal():
    return 1000
def get_problem_goal():
    return 2503
