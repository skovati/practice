class Solution:
    def topKFrequent(self, nums: list[int], k: int) -> list[int]:
        count = {}
        for n in nums:
            count[n] = 1 + count.get(n, 0)

        # freq stores a mapping of how many times we saw something to it's values
        freq = [ [] for _ in range(len(nums) + 1) ]

        for n, c in count.items():
            freq[c].append(n)

        res = []
        for elem in freq[::-1]:
            res.extend(elem)
            if len(res) == k:
                return res


assert(Solution().topKFrequent(nums = [1,2,2,3,3,3], k = 2) == [2, 3])
