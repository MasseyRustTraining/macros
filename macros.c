#include <stdio.h>

#define DOUBLE(X) ((X)+(X))
#define SQUARE(X) ((X)*(X))
#define CONCAT(X, Y) X Y

extern int a[DOUBLE(5)];

int main(void) {
    printf("%d\n", SQUARE(2));
    printf("%s\n", CONCAT("x", "y"));
    return 0;
}
