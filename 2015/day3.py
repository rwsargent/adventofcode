def readinput():
    with open ("day3input.txt", "r") as source:
        return source.read().replace('\n', '')

def execute_part_one(input):
    houses = set()
    santa = Santa()
    houses.add(House(0,0))
    for direction in input:
        santa.move(direction)
        houses.add(santa.get_house())
    return len(houses)
        
def execute_part_two(input):
    houses = set()
    houses.add(House(0, 0))
    santa = Santa()
    robot = Santa()
    santasTurn = False
    for direction in input:
        current_santa = santa if santasTurn else robot
        santasTurn = not santasTurn
        current_santa.move(direction)
        houses.add(current_santa.get_house())
    return len(houses)
            
class Santa:
    def __init__(self):
       self.x = 0
       self.y = 0
    def move(self, direction):
        if (direction == '<'):
            self.x -= 1
        elif(direction == '>'):
            self.x += 1
        elif(direction == '^'):
            self.y += 1
        elif(direction == 'v'):
            self.y -= 1
    def get_house(self):
        return House(self.x, self.y)
        

class House:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        if not isinstance(other, House):
            return False
        return other.x == self.x and other.y == self.y
    def __hash__(self):
        hashCode = (self.x + self.y)
        return (hashCode<<5) - hashCode
