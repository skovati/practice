/*

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

*/

public class Problem_10 {

	public static boolean checkPrime(long p) {
		boolean prime = true;
		for(long k = 3; k < Math.sqrt(p); k++) {
			if (p==2) {
				break;
			}
			if((p % k) == 0) {
				return false;
			}
		}
		return prime;
	}

	public static void main(String[] args) {

		long sum = 0;
		long startTime = System.nanoTime();

		for (long l = 1; l < 2000000; l+=2) {
			if(checkPrime(l)) {
				sum = sum + l;
			}
		}
		long endTime   = System.nanoTime();
		long totalTime = endTime - startTime;
		System.out.println(sum);
		System.out.println(totalTime/1000000000);
	}
}