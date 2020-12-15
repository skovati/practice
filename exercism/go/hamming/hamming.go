package hamming

import "errors"

func Distance(a, b string) (int, error) {
    if len(a) != len(b) {return 0, errors.New("must be of same length")}
    counter := 0
    for i, v := range a {
        if byte(v) != b[i] {counter++}
    }
    return counter, nil
}
