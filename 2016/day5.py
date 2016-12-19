import hashlib
def read_input(filename):
    with open(filename) as file:
        for line in file:
            pass
    return "wtnhxymk"

def execute_part_one(input):
    postfix = 1
    password = ""
    while(len(password) < 8): # should have a kill switch
        md5 = hashlib.md5()
        md5.update(input + str(postfix))
        md5_hash = md5.hexdigest();
        if (md5_hash.startswith('00000')):
            password += md5_hash[5]
        postfix += 1
    return password

def execute_part_two(input):
    postfix = 1
    password = [" " for i in range(8)]
    while(" " in password): 
        md5 = hashlib.md5()
        md5.update(input + str(postfix))
        md5_hash = md5.hexdigest();
        if (md5_hash.startswith('00000')):
            pass_position = md5_hash[5]
            if not (ord(pass_position) > ord('8')):
              pass_idx = int(pass_position)  
              if password[pass_idx] == " ":
                password[pass_idx] = md5_hash[6]
        print("\rCurrent password: [" + str("".join(password)) + "] trying hash " + str(md5_hash)),
        postfix += 1
    return "".join(password)

def get_expected_results_map():
    return { 'test.txt' : (None, None)}

