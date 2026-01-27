/*
 * 06_arrays.c - Fixed-size arrays
 *
 * Arrays are contiguous blocks of memory holding elements of the same type.
 * Array size must be known at compile time (in standard C99).
 */

#include <stdio.h>

#define ARRAY_SIZE 5

/* Function to print array - note: arrays decay to pointers when passed */
void print_array(const int *arr, int size) {
    printf("[ ");
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("]\n");
}

/* Function to sum array elements */
int sum_array(const int *arr, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

int main(void) {
    /* Array declaration with size */
    int numbers[ARRAY_SIZE];

    /* Array with initializer */
    int primes[] = { 2, 3, 5, 7, 11 };  /* Size inferred from initializer */

    /* Partial initialization - rest are zero */
    int partial[10] = { 1, 2, 3 };  /* Elements 3-9 are 0 */

    /* All zeros */
    int zeros[5] = { 0 };

    printf("=== Array Basics ===\n");

    /* Initialize numbers array */
    for (int i = 0; i < ARRAY_SIZE; i++) {
        numbers[i] = (i + 1) * 10;
    }

    printf("numbers: ");
    print_array(numbers, ARRAY_SIZE);
    printf("Sum: %d\n", sum_array(numbers, ARRAY_SIZE));

    printf("\nprimes: ");
    print_array(primes, sizeof(primes) / sizeof(primes[0]));

    printf("\npartial (size 10, initialized with {1,2,3}): ");
    print_array(partial, 10);

    printf("\nzeros: ");
    print_array(zeros, 5);

    /* sizeof on arrays */
    printf("\n=== Array Sizes ===\n");
    printf("sizeof(numbers) = %zu bytes\n", sizeof(numbers));
    printf("sizeof(numbers[0]) = %zu bytes\n", sizeof(numbers[0]));
    printf("Number of elements = %zu\n", sizeof(numbers) / sizeof(numbers[0]));

    /* 2D arrays */
    printf("\n=== 2D Arrays ===\n");
    int matrix[3][4] = {
        { 1, 2, 3, 4 },
        { 5, 6, 7, 8 },
        { 9, 10, 11, 12 }
    };

    printf("3x4 matrix:\n");
    for (int row = 0; row < 3; row++) {
        printf("  ");
        for (int col = 0; col < 4; col++) {
            printf("%3d ", matrix[row][col]);
        }
        printf("\n");
    }

    /* Array bounds - C does NOT check! */
    printf("\n=== Warning: No Bounds Checking ===\n");
    printf("C allows out-of-bounds access (undefined behavior).\n");
    printf("Always track array sizes carefully!\n");

    return 0;
}
