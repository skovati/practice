package main

func lengthOfLongestSubstring(s string) int {
    // counter to keep track of longest string
    var final int = 0
    for i := range s {
        // make map to keep track of seen runes
        // this gets reset each loop
        seen := make(map[rune]int)

        // counter for this starting rune v (s[i])
        // also reset each loop
        var counter int = 0

        // make slice of runes from i to end
        // since this is just the runes we need to check for
        // longer substrings than previously found
        // (starting at s[i])
        runes := []rune(s[i:])
        // loop over these runes, and check if rune was seen before
        for _, v := range runes {
            // if not seen before, increament counter and continue
            if _, ok := seen[v]; !ok {
                counter++
                seen[v] = v
            } else {
                // otherwise, substring is terminated
                // break to go to next rune in original string
                break
            }
        }
        // if counter after loop is larger than global final, update it
        if counter > final {
            final = counter
        }
    }
    return final
}
