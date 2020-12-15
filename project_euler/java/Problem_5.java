/*

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

*/
public class Problem_5 {
	public static void main(String[] args) {

		final long cap = 999999999;
		int counter = 0;
		long result = 0;

		for(int i = 21; i < cap; i++) {
			counter = 0;
			for(int j = 1; j < 21; j++) {
				if((i % j) == 0) {
					counter++;
				}
			}
			if(counter == 20){
				result = i;
				break;
			}
		}
		System.out.println("The smallest number evenly divisible by all numbers 1-20 is: " + result);
		

	}
}