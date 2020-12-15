#include <stdio.h>

int main() {

    int prev = 0;
    int temp;
    int num = 1;
    int sum = 0;
    int limit = 4000000;

    while(num < limit) {
        temp = prev;
        prev = num;
        num += temp;
        if(num%2 == 0 && num < limit) {
            sum += num;
        }
    }
    printf("%d", sum);

}

