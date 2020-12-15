// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package bob should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package bob

import (
    "strings"
)

// Hey should have a comment documenting it.
func Hey(remark string) string {
    remark = strings.TrimSpace(remark)
    question := false
    caps := false
    empty := false
    nums := false

    if !strings.ContainsAny(remark , "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXWYZ") {
        nums = true
    }
    if strings.Index(remark, "?") == (len(remark)-1) {
        question = true
    }
    if strings.ToUpper(remark) == remark {
        caps = true
    }
    if remark == "" {
        empty = true
    }
    switch {
    case empty:
        return "Fine. Be that way!"
    case caps && question && !nums:
        return "Calm down, I know what I'm doing!"
    case caps && !nums:
        return "Whoa, chill out!"
    case question:
        return "Sure."
    }
    return "Whatever."
}
