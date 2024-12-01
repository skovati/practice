class Solution:
    """
    You are given an integer array prices where prices[i] is the price of NeetCoin on the ith day.

    You may choose a single day to buy one NeetCoin and choose a different day in the future to sell it.

    Return the maximum profit you can achieve. You may choose to not make any transactions, in which case the profit would be 0.

    Example 1:

    Input: prices = [10,1,5,6,7,1]

    Output: 6

    Explanation: Buy prices[1] and sell prices[4], profit = 7 - 1 = 6.

    Example 2:

    Input: prices = [10,8,7,5,2]

    Output: 0

    Explanation: No profitable transactions can be made, thus the max profit is 0.

    Constraints:

        1 <= prices.length <= 100
        0 <= prices[i] <= 100
    """
    def maxProfit(self, prices: list[int]) -> int:
        """
        Two pointers:
        - first one keeps track of lowest price seen so far
        - second one keeps track of current sale candidate

        Then just keep track of max profit (sale_candidate - lowest_buy)

        """
        max_profit = 0
        lowest_buy = prices[0]

        for sale_candidate in prices:
            lowest_buy = min(lowest_buy, sale_candidate)
            max_profit = max(max_profit, sale_candidate - lowest_buy)

        return max_profit

sol = Solution()
assert sol.maxProfit([10, 1, 5, 6, 7 ,1]) == 6
assert sol.maxProfit([10, 8, 7,  5, 2]) == 0
assert sol.maxProfit([1, 4, 2]) == 3
assert sol.maxProfit([2,1,2,0,1]) == 1
