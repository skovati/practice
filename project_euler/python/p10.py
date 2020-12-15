# The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

# Find the sum of all the primes below two million.
import math
def main():
    sum = 0
    for i in range(2, 2000000):
        if(is_prime(i)):
            sum += i
    print(sum)

def is_prime(n):
    if(n == 2):
        return True
    if(n%2 == 0):
        return False
    if(n > 1):
        i = 2
        # loops from 2 -> sqrt(n)
        while(i**2 <= n):
            if(n%i == 0):
                return False
            i += 1
    return True


if __name__ == "__main__":
    main()
