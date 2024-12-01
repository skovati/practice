package day01

import (
    "fmt"
    "os"
    "slices"
    "strconv"
    "strings"
)

func read_lines_to_list(path string) []string {
    contents, err := os.ReadFile(path)
    if err != nil {
        fmt.Printf("Warning: can't read file %s\n", path)
        return []string{}
    }

    return strings.Split(string(contents), "\n")
}

func split_by_spaces(str string) (int, int, error) {
    tokens := strings.Fields(str)
    ints := []int{}
    for _, v := range tokens {
        // fmt.Printf("reading token %s\n", v)
        int, err := strconv.Atoi(v)
        if err != nil {
            fmt.Printf("Can't convert the following line to ints: %s", tokens)
            continue
        }
        ints = append(ints, int)
        // fmt.Printf("Dumping ints contents: %d\n", ints)
    }
    if len(ints) != 2 {
        return 0, 0, fmt.Errorf("Couldn't find two ints for the following line: %s", tokens)
    }
    return ints[0], ints[1], nil
}

func solve_p1(input_path string) int {
    lines := read_lines_to_list(input_path)
    left := []int{}
    right := []int{}

    for _, v := range lines {
        l, r, err := split_by_spaces(v)
        if err != nil {
            continue
        }
        left = append(left, l)
        right = append(right, r)
    }

    if len(left) != len(right) {
        panic(fmt.Sprintf("Lists are not same length: %d vs %d", len(left), len(right)))
    }

    slices.Sort(left)
    slices.Sort(right)

    sum := 0

    for i := range left {
        diff := right[i] - left[i]
        if diff < 0 {
            diff = left[i] - right[i]
        }
        sum += diff
    }

    return sum
}

func solve_p2(input_path string) int {
    lines := read_lines_to_list(input_path)
    left := []int{}
    right := []int{}

    for _, v := range lines {
        l, r, err := split_by_spaces(v)
        if err != nil {
            continue
        }
        left = append(left, l)
        right = append(right, r)
    }

    if len(left) != len(right) {
        panic(fmt.Sprintf("Lists are not same length: %d vs %d", len(left), len(right)))
    }

    right_map := make(map[int]int)
    for _, v := range right {
        right_map[v] = right_map[v] + 1
    }

    sum := 0
    for _, v := range left {
        sum += v * right_map[v]
    }

    return sum

}
