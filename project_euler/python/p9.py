def main():
    for a in range(999):
        for b in range(999):
            c = 1000 - b - a
            if(is_triplet(a, b, c)):
                print(a*b*c)
                return

def is_triplet(a, b, c):
    return (a < b < c) and (a**2 + b**2 == c**2)

if __name__ == "__main__":
    main()
