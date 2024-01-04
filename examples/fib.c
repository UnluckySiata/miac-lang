#include <stdio.h>
int fib(int x) {
if (x == 0 || x == 1) {
return 1;
}
return fib(x - 1) + fib(x - 2);
}
int main() {
const int x = 5;
int a = fib(x);
const int q = printf("fib %d: %d\n", x, a);
return 0;
}
