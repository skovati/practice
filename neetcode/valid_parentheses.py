class Solution:
    """
    Validate Parentheses

    You are given a string s consisting of the following characters: '(', ')', '{', '}', '[' and ']'.

    The input string s is valid if and only if:

        Every open bracket is closed by the same type of close bracket.
        Open brackets are closed in the correct order.
        Every close bracket has a corresponding open bracket of the same type.

    Return true if s is a valid string, and false otherwise.

    Example 1:

    Input: s = "[]"

    Output: true

    Example 2:

    Input: s = "([{}])"

    Output: true

    Example 3:

    Input: s = "[(])"

    Output: false

    Explanation: The brackets are not closed in the correct order.

    Constraints:

        1 <= s.length <= 1000
    """
    def isValid(self, s: str) -> bool:
        seen = []

        for c in s:
            print(f"c: {c}")
            print(f"seen: {seen}")
            if c in {"{", "(", "["}:
                seen.append(c)
            elif c == "}":
                if len(seen) == 0 or seen.pop() != "{":
                    return False
            elif c == "]":
                if len(seen) == 0 or seen.pop() != "[":
                    return False
            elif c == ")":
                if len(seen) == 0 or seen.pop() != "(":
                    return False
            else:
                return False

        return len(seen) == 0

sol = Solution()
assert sol.isValid("[]") == True
