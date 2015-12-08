import pdb
from operator import mul
def readinput():
    boxes = []
    with open("day2input.txt") as file:
        for line in file:
            boxes.append(createBox(line))
    return boxes
        
def execute_part_one(input):
    totalPaper = 0
    for box in input:
        totalPaper += box.get_total_wrapping_paper()
    return totalPaper

def execute_part_two(input):
    totalRibbon = 0
    for box in input:
        totalRibbon += box.get_ribbon()
    return totalRibbon

def createBox(line):
    w, h, l = line.split("x")
    return Box(int(w), int(h), int(l))

class Box:
    def __init__(self, width, height, length):
        self.dimensionList = [width, height, length]

    def get_total_wrapping_paper(self):
        boxPaper = 0
        smallestSide = 9999999
        for dim in range(len(self.dimensionList)):
            dim1 = self.dimensionList[dim]
            dim2 = self.dimensionList[((dim+1)%len(self.dimensionList))] # wrap around when at the end
            smallestSide = min(smallestSide, dim1*dim2)
            boxPaper += 2*dim1*dim2
            
        return boxPaper + smallestSide

    def get_ribbon(self):
        bow = reduce(mul, self.dimensionList, 1)
        twoSmallest = list(self.dimensionList)
        twoSmallest.remove(max(twoSmallest))
        wrap = reduce(lambda acc, dimension: acc + (dimension + dimension), twoSmallest, 0)
        return bow + wrap
