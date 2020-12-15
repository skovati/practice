/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

*/
import java.util.Scanner;
public class Problem_3 {

	public static boolean checkPrime(long p) {
		boolean prime = true;
		for(long k = 2; k < Math.sqrt(p); k++) {
			if((p % k) == 0) {
				prime = false;
			}
		}
		return prime;
	}

	public static void main(String[] args) {

		Scanner input = new Scanner(System.in);
		long limit = input.nextLong();
		long maxPrime = 0;

		for(long i =1; i < (Math.sqrt(limit)); i++) {

			if((limit % i) == 0) {
				if(checkPrime(i) == true) {
					if(i > maxPrime) {
						maxPrime = i;
					}
				}
			}
		}
		System.out.println(maxPrime);

	}
}