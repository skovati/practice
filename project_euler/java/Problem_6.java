/*
Find the difference between the sum of the squares of the first one hundred natural numbers 
and the square of the sum.
*/

public class Problem_6 {
	public static void main(String[] args) {

		long sumSquare = 0;
		long squareSum = 0;

		for(int i = 1; i < 101; i++) {
			sumSquare = sumSquare + (i*i);
			squareSum = squareSum + i;
		}

		squareSum = squareSum * squareSum;
		System.out.println("Sum of squares is: " + sumSquare);
		System.out.println("Sqaure of sum is: " + squareSum);

		System.out.println("The difference is " + (squareSum - sumSquare));

	}
}