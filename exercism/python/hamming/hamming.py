def distance(strand_a, strand_b):
    hamming = 0;
    if (len(strand_a) != len(strand_b)):
        raise Exception("Error, strands must be of same length")
        return hamming
    for a, b in zip(strand_a, strand_b):
        if (a != b): hamming += 1
    return hamming
