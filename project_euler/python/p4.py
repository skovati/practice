def check_palidrome(a):
    a = str(a)
    strlen = len(a)
    if(a == a[::-1]):
        return True
    return False

def run():
    largest = 0;
    for i in range(999):
        for j in range(999):
            if(check_palidrome(i*j) and i*j > largest):
                largest = i*j
    print(largest)

run()


