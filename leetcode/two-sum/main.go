package main

import (
    // "fmt"
)

func twoSum(nums []int, target int) []int {
    // init hash table
    m := make(map[int]int)

    // loop over nums
    for i, v := range nums {
        // calc diff, we need to check if this number is in map
        diff := target - v
        // get index and bool contains
        index, ok := m[diff]
        // if diff is in map and not the same as current loop,
        // then we have our answer, return formatted array
        if ok {
            return []int{i, index}
        }
        // otherwise, add this to map for future checking
        m[v] = i
    }

    // otherwise, not found (this should never happen given constraints)
    return []int{}
}
