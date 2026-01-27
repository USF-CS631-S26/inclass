/*
 * 07_strings.c - Char arrays, null termination, strnlen
 *
 * C strings are arrays of char terminated by '\0' (null character).
 * Always ensure space for the null terminator!
 */

#include <stdio.h>
#include <string.h>

int main(void) {
    /* String literal - stored in read-only memory */
    const char *greeting = "Hello";

    /* Character array - modifiable, explicitly showing null terminator */
    char word[6] = { 'H', 'e', 'l', 'l', 'o', '\0' };

    /* Character array initialized from string literal */
    char buffer[20] = "World";  /* Remaining chars are '\0' */

    printf("=== String Basics ===\n");
    printf("greeting: \"%s\"\n", greeting);
    printf("word: \"%s\"\n", word);
    printf("buffer: \"%s\"\n", buffer);

    /* String length vs array size */
    printf("\n=== Length vs Size ===\n");
    printf("strlen(buffer) = %zu (characters before '\\0')\n", strlen(buffer));
    printf("sizeof(buffer) = %zu (total array size)\n", sizeof(buffer));

    /* strnlen - safe length with maximum */
    printf("\n=== strnlen (Safe Length) ===\n");
    printf("strnlen(buffer, 20) = %zu\n", strnlen(buffer, 20));
    printf("strnlen(buffer, 3) = %zu (limited)\n", strnlen(buffer, 3));

    /* String copying */
    printf("\n=== String Copying ===\n");
    char dest[20];

    /* strncpy - safe copy with size limit */
    strncpy(dest, "Hello, World!", sizeof(dest) - 1);
    dest[sizeof(dest) - 1] = '\0';  /* Ensure null termination */
    printf("After strncpy: \"%s\"\n", dest);

    /* String concatenation */
    printf("\n=== String Concatenation ===\n");
    char result[50] = "Hello";
    strncat(result, ", ", sizeof(result) - strlen(result) - 1);
    strncat(result, "World!", sizeof(result) - strlen(result) - 1);
    printf("Concatenated: \"%s\"\n", result);

    /* String comparison */
    printf("\n=== String Comparison ===\n");
    const char *s1 = "apple";
    const char *s2 = "banana";
    const char *s3 = "apple";

    printf("strcmp(\"%s\", \"%s\") = %d\n", s1, s2, strcmp(s1, s2));
    printf("strcmp(\"%s\", \"%s\") = %d\n", s2, s1, strcmp(s2, s1));
    printf("strcmp(\"%s\", \"%s\") = %d (equal)\n", s1, s3, strcmp(s1, s3));

    /* Iterating through string characters */
    printf("\n=== Character Iteration ===\n");
    const char *str = "Hello";
    printf("Characters in \"%s\": ", str);
    for (const char *p = str; *p != '\0'; p++) {
        printf("'%c' ", *p);
    }
    printf("\n");

    /* Common pitfall: forgetting null terminator */
    printf("\n=== Common Pitfall ===\n");
    char bad[5] = { 'H', 'e', 'l', 'l', 'o' };  /* No null terminator! */
    printf("Array without '\\0' - don't use with string functions!\n");
    printf("sizeof(bad) = %zu, but no safe strlen possible.\n", sizeof(bad));
    (void)bad;

    return 0;
}
