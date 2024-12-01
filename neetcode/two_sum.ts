export const twoSum = (nums: number[], target: number): number[] => {

    const pair = new Map<number, number>();

    for (let i = 0; i < nums.length; i++) {
        const num = nums[i];
        const candidate = target - num;
        if (pair.has(candidate)) {
            return [pair.get(candidate) || 0, i];
        }
        pair.set(num, i);
    }

    return [0, 0];
}
