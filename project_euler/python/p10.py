# The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

# Find the sum of all the primes below two million.
import math
def main():
    sum = 0
    for i in range(2, 2000000):
        if(is_prime(i)):
            sum += i
    print(sum)

def is_prime(num):
    if(num > 1):
        if(num%2 == 0):
            return False
        if(num%3 == 0):
            return False
        for i in range(3, math.ceil(num**0.5), 2):
            if(num%i == 0):
                return False
    return True


if __name__ == "__main__":
    main()
