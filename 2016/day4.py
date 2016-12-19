from collections import namedtuple, Counter
import re
def read_input(filename):
    regex = re.compile('([a-z-]+)(\d{3})\[([a-z]{5})\]')
    with open(filename) as file:
        rooms = []
        for line in file:
            room = namedtuple("Room", "name id checksum")
            result = regex.match(line)
            room.name = result.group(1)
            room.id =result.group(2)
            room.checksum = result.group(3)
            rooms.append(room)
    return rooms
        
def execute_part_one(rooms):
    sectorSum = 0
    #five most common letters
    for room in rooms:
        if isRealRoom(room):
            sectorSum += int(room.id)
    return sectorSum

def execute_part_two(rooms):
    real_rooms = filter(isRealRoom, rooms)
    for room in real_rooms:
        decyphered = ""
        for letter in room.name:
            decyphered += decypher(letter, int(room.id) % 26)
        if "orth" in decyphered:
            print(decyphered + " " + str(room.id))
    return None

def get_expected_results_map():
    return { 'test.txt' : (1514, None)}

def decypher(letter, distance):
    if letter == "-":
        return " "
    cypher_code = ord(letter) - ord('a') # keep things [0, 26)
    plaintext_code = ((cypher_code + distance) % 26) + ord('a') 
    return chr(plaintext_code) # turn the ascii value back into a string

def isRealRoom(room):
    counts = Counter(room.name.replace("-", ""))
    top_five = sorted(sorted(counts.most_common(), key=lambda x: x[0]), key=lambda x: x[1], reverse=True)[:5]
    letters = "".join([common[0] for common in top_five])
    return letters == room.checksum
