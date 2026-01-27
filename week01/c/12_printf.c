/*
 * 12_printf.c - Format specifiers
 *
 * printf() uses format specifiers to control output:
 *   %d, %i  - signed integer
 *   %u      - unsigned integer
 *   %x, %X  - hexadecimal (lower/upper)
 *   %o      - octal
 *   %f      - float/double
 *   %e, %E  - scientific notation
 *   %c      - character
 *   %s      - string
 *   %p      - pointer
 *   %%      - literal percent sign
 */

#include <stdio.h>

int main(void) {
    /* ==================== INTEGER FORMATS ==================== */
    printf("=== Integer Formats ===\n");
    int num = 42;
    int neg = -17;
    unsigned int u = 3000000000U;

    printf("%%d:  %d\n", num);
    printf("%%i:  %i (same as %%d)\n", num);
    printf("%%d:  %d (negative)\n", neg);
    printf("%%u:  %u (unsigned)\n", u);

    /* Different bases */
    printf("\nNumber 255 in different bases:\n");
    printf("%%d:  %d (decimal)\n", 255);
    printf("%%x:  %x (hex lowercase)\n", 255);
    printf("%%X:  %X (hex uppercase)\n", 255);
    printf("%%o:  %o (octal)\n", 255);

    /* With 0x prefix for hex */
    printf("%%#x: %#x (with 0x prefix)\n", 255);

    /* ==================== WIDTH AND PADDING ==================== */
    printf("\n=== Width and Padding ===\n");
    printf("|%5d| (width 5, right-aligned)\n", 42);
    printf("|%-5d| (width 5, left-aligned)\n", 42);
    printf("|%05d| (width 5, zero-padded)\n", 42);

    printf("\nTable alignment:\n");
    printf("%10s %10s\n", "Name", "Score");
    printf("%10s %10d\n", "Alice", 95);
    printf("%10s %10d\n", "Bob", 87);
    printf("%10s %10d\n", "Charlie", 92);

    /* ==================== FLOATING POINT ==================== */
    printf("\n=== Floating Point ===\n");
    double pi = 3.14159265358979;
    double big = 1234567890.123;
    double small = 0.000012345;

    printf("%%f:   %f (default 6 decimals)\n", pi);
    printf("%%.2f: %.2f (2 decimals)\n", pi);
    printf("%%.10f: %.10f (10 decimals)\n", pi);
    printf("%%e:   %e (scientific)\n", big);
    printf("%%E:   %E (scientific uppercase)\n", small);
    printf("%%g:   %g (auto format)\n", pi);

    /* Width with float */
    printf("|%10.2f| (width 10, 2 decimals)\n", pi);

    /* ==================== CHARACTERS AND STRINGS ==================== */
    printf("\n=== Characters and Strings ===\n");
    char ch = 'A';
    const char *str = "Hello, World!";

    printf("%%c:  %c\n", ch);
    printf("%%d of char: %d (ASCII value)\n", ch);
    printf("%%s:  %s\n", str);
    printf("%%.5s: %.5s (first 5 chars)\n", str);
    printf("%%20s: |%20s| (width 20)\n", str);
    printf("%%-20s: |%-20s| (left-aligned)\n", str);

    /* ==================== POINTERS ==================== */
    printf("\n=== Pointers ===\n");
    int x = 42;
    int *ptr = &x;

    printf("%%p:  %p\n", (void *)ptr);

    /* ==================== SPECIAL ==================== */
    printf("\n=== Special ===\n");
    printf("Percent sign: %%\n");
    printf("Tab: |\t|\n");
    printf("Newline: |\\n|\n");

    /* ==================== SIZE SPECIFIERS ==================== */
    printf("\n=== Size Specifiers ===\n");
    long l = 1234567890L;
    long long ll = 9223372036854775807LL;
    size_t sz = sizeof(int);

    printf("%%ld:  %ld (long)\n", l);
    printf("%%lld: %lld (long long)\n", ll);
    printf("%%zu:  %zu (size_t)\n", sz);

    /* ==================== COMBINING SPECIFIERS ==================== */
    printf("\n=== Formatted Table ===\n");
    printf("+-------+----------+--------+\n");
    printf("| %-5s | %8s | %6s |\n", "ID", "Name", "Score");
    printf("+-------+----------+--------+\n");
    printf("| %-5d | %8s | %6.1f |\n", 1, "Alice", 95.5);
    printf("| %-5d | %8s | %6.1f |\n", 2, "Bob", 87.3);
    printf("| %-5d | %8s | %6.1f |\n", 3, "Charlie", 92.8);
    printf("+-------+----------+--------+\n");

    return 0;
}
