// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
// 
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

package main

type ListNode struct {
    Val int
    Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
    // dummy header node, will return dummy.Next
    var dummy *ListNode = &ListNode{Val: 0, Next: nil}
    current := dummy
    var x int
    var y int
    var carry int
    var sum int
    for l1 != nil || l2 != nil {
        // if l1 not nil, use its val
        if l1 != nil {
            x = l1.Val
        } else {
            // otherwise, assign to 0
            x = 0
        }

        // if l2 not nil, use its val
        if l2 != nil {
            y = l2.Val
        } else {
            // otherwise, assign to 0
            y = 0
        }
        sum = carry + x + y
        carry = sum / 10
        current.Next = &ListNode{Val: sum % 10}
        current = current.Next
        if l1 != nil {
            l1 = l1.Next
        }

        if l2 != nil {
            l2 = l2.Next
        }
    }
    if carry > 0 {
        current.Next = &ListNode{Val: carry}
    }
    return dummy.Next
}
