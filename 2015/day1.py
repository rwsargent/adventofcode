import sys
def whatFloor(input):
    floor = 0;
    for char in input:
        floor = upOrDown(char, floor)
    return floor

def upOrDown(char, floor):
    if(char == '('):
        return floor + 1
    elif(char == ')'):
        return floor - 1

def firstBasement(input):
    floor = 0
    for charIdx in range(len(input)):
        floor = upOrDown(input[charIdx], floor)
        if(floor == -1):
            return charIdx + 1 #for some reason this is one based.

def main():
    if len(sys.argv) != 2:
        print("Please supply file name")
        return
    filestring = ""
    with open (sys.argv[1], "r") as myfile:
        filestring=myfile.read().replace('\n', '')
#    print(whatFloor(filestring))
    print(firstBasement(filestring))


if __name__ == "__main__":
    main()

