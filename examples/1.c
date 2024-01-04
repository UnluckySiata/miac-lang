#include <stdio.h>
int f(int a, int b) {
return a + b;
}
int main() {
char * a = "aaaaaa";
float x = 3.1 / 2.0 * 1.0;
const int b = 1;
x = f(1, 2);
if (x > 2) {
}
while (!x) {
x = x + 1;
}
int q = printf("%f\n", x);
return x <= 1;
}
