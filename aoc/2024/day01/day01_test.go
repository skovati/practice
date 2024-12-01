package day01

import (
    "testing"
)

func TestExampleP1(t *testing.T) {
    expected := 11
    result := solve_p1("example.txt")
    if result != expected {
        t.Fatalf("Expected %d, got %d", expected, result)
    }
}

func TestExampleP2(t *testing.T) {
    expected := 31
    result := solve_p2("example.txt")
    if result != expected {
        t.Fatalf("Expected %d, got %d", expected, result)
    }
}

func TestPart1(t *testing.T) {
    expected := 1889772
    result := solve_p1("input.txt")
    if result != expected {
        t.Fatalf("Expected %d, got %d", expected, result)
    }
}

func TestPart2(t *testing.T) {
    expected := 23228917
    result := solve_p2("input.txt")
    if result != expected {
        t.Fatalf("Expected %d, got %d", expected, result)
    }
}
