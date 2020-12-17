from math import *

def divs(n):
    div = 0
    for i in range(1, ceil(sqrt(n))):
        if n%i == 0:
            div += 2
        if i*i == n:
            div -= 1
    return div

for tri_num in range(100000):
    tri_val = tri_num * (tri_num +1)/2
    if divs(tri_val) > 500:
        print(tri_val)
        break
