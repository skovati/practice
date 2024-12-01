from pprint import pformat
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    """
    Reverse a Linked List

    Given the beginning of a singly linked list head, reverse the list, and return the new beginning of the list.

    Example 1:

    Input: head = [0,1,2,3]

    Output: [3,2,1,0]

    Example 2:

    Input: head = []

    Output: []

    Constraints:

        0 <= The length of the list <= 1000.
        -1000 <= Node.val <= 1000

    """
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return None

        curr = head
        prev = None

        while curr is not None:
            print(pformat(curr))
            print(pformat(prev))
            temp = curr.next
            curr.next = prev
            prev = curr
            curr = temp

        return prev

sol = Solution()

head = ListNode(
    0,
    ListNode(
        1,
        ListNode(
            2,
            ListNode(3)
        )
    )
)

sol.reverseList(head)
