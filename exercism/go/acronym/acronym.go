// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import (
    "strings"
)

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) (acro string) {
    s = strings.ReplaceAll(s, "-", " ")
    s = strings.ReplaceAll(s, "_", " ")
    strs := strings.Fields(s)
    for _, v := range strs {
        if len(v) > 0 {
            acro += strings.ToUpper(string(v[0]))
        }
    }
    return acro
}
