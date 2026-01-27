/*
 * 05_pointer_arith.c - Incrementing/traversing with pointers
 *
 * Pointer arithmetic automatically accounts for the size of
 * the pointed-to type. Adding 1 to an int* advances by sizeof(int).
 */

#include <stdio.h>

int main(void) {
    int numbers[] = { 10, 20, 30, 40, 50 };
    int *ptr = numbers;  /* Array name decays to pointer to first element */

    printf("=== Array Traversal with Pointer Arithmetic ===\n");
    printf("Array: { 10, 20, 30, 40, 50 }\n\n");

    /* Accessing elements via pointer arithmetic */
    printf("ptr       = %p, *ptr     = %d\n", (void *)ptr, *ptr);
    printf("ptr + 1   = %p, *(ptr+1) = %d\n", (void *)(ptr+1), *(ptr+1));
    printf("ptr + 2   = %p, *(ptr+2) = %d\n", (void *)(ptr+2), *(ptr+2));

    /* Note the address difference */
    printf("\nAddress difference (ptr+1) - ptr = %ld bytes\n",
           (char *)(ptr+1) - (char *)ptr);
    printf("This equals sizeof(int) = %zu\n", sizeof(int));

    /* Traversing with increment */
    printf("\n=== Traversing with ptr++ ===\n");
    ptr = numbers;  /* Reset to beginning */
    for (int i = 0; i < 5; i++) {
        printf("*ptr = %d\n", *ptr);
        ptr++;  /* Move to next element */
    }

    /* Common idiom: iterate while pointer < end */
    printf("\n=== Pointer Comparison Loop ===\n");
    int *start = numbers;
    int *end = numbers + 5;  /* One past the last element */

    for (int *p = start; p < end; p++) {
        printf("%d ", *p);
    }
    printf("\n");

    /* Pointer subtraction gives element count */
    printf("\n=== Pointer Subtraction ===\n");
    printf("end - start = %ld elements\n", end - start);

    /* Working with char pointers (strings) */
    printf("\n=== Character Pointer Arithmetic ===\n");
    const char *str = "Hello";
    const char *s = str;
    while (*s != '\0') {
        printf("'%c' at %p\n", *s, (void *)s);
        s++;
    }

    /* Bracket notation is pointer arithmetic in disguise */
    printf("\n=== Equivalence: arr[i] == *(arr + i) ===\n");
    printf("numbers[2] = %d\n", numbers[2]);
    printf("*(numbers + 2) = %d\n", *(numbers + 2));
    /* Even this works (but don't do it!): */
    printf("2[numbers] = %d  (weird but valid!)\n", 2[numbers]);

    return 0;
}
