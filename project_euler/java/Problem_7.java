/*

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

*/


public class Problem_7 {

	public static boolean checkPrime(long p) {
		boolean prime = true;
		for(long k = 2; k < p; k++) {
			if((p % k) == 0) {
				prime = false;
			}
		}
		return prime;
	}

	public static void main(String[] args) {

		int counter = 0;
		long i = 2;
		long prime = 0;
		while (counter < 10001) {
			if (checkPrime(i)) {
				counter++;
				prime = i;
			}
			i++;
		}
		System.out.println(prime);


	}
}