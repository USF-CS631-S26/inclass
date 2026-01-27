/*
 * 01_defines.c - Preprocessor #define for constants
 *
 * The #define directive creates compile-time constants.
 * The preprocessor replaces all occurrences before compilation.
 */

#include <stdio.h>

/* Simple constant definitions */
#define MAX_BUFFER_SIZE 256
#define PI 3.14159
#define GREETING "Hello, World!"

/* Macro with expression (use parentheses for safety) */
#define SQUARE(x) ((x) * (x))

/* Multi-line macro using backslash continuation */
#define PRINT_BOUNDS(name, val, max) \
    printf("%s = %d (max: %d)\n", name, val, max)

int main(void) {
    /* Using defined constants */
    char buffer[MAX_BUFFER_SIZE];
    double radius = 5.0;
    double area = PI * SQUARE(radius);

    printf("Buffer size: %d bytes\n", MAX_BUFFER_SIZE);
    printf("%s\n", GREETING);
    printf("Area of circle with radius %.1f: %.2f\n", radius, area);

    /* Demonstrating SQUARE macro */
    int x = 4;
    printf("Square of %d is %d\n", x, SQUARE(x));

    /* Using multi-line macro */
    int count = 42;
    PRINT_BOUNDS("count", count, MAX_BUFFER_SIZE);

    /* Why parentheses matter in macros:
     * SQUARE(1+2) expands to ((1+2) * (1+2)) = 9
     * Without parens: 1+2 * 1+2 = 1+2+2 = 5 (wrong!)
     */
    printf("SQUARE(1+2) = %d\n", SQUARE(1+2));

    (void)buffer; /* Suppress unused variable warning */
    return 0;
}
