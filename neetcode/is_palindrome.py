def isPalindrome(s: str) -> bool:
    s = "".join(c for c in s if c.isalnum())
    s = s.lower()

    p1 = 0
    p2 = len(s) - 1

    if len(s) == 1:
        return False

    while p1 < len(s) and p2 >= 0:
        if s[p1] != s[p2]:
            return False
        p1 += 1
        p2 -= 1

    return True

print(isPalindrome("cac"))
print(isPalindrome("ca c"))
print(isPalindrome("c9c"))
print(isPalindrome("c00"))
print(isPalindrome("car"))
