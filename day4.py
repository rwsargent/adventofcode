import hashlib
def read_input(filename):
    with open(filename) as file:
        return file.read().strip()
        
def execute_part_one(input):
    postfix = 1
    while(True): # should have a kill switch
        md5 = hashlib.md5()
        md5.update(input + str(postfix))
        if (md5.hexdigest().startswith('00000')):
            return postfix
        postfix += 1

def execute_part_two(input):
    postfix = 1
    while(True): # should have a kill switch
        md5 = hashlib.md5()
        md5.update(input + str(postfix))
        if (md5.hexdigest().startswith('000000')):
            return postfix
        postfix += 1


def get_expected_results_map():
    return { 'test1.txt' : (609043, None), 'test2.txt' : (1048970,None)}
