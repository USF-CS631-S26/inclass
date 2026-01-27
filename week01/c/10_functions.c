/*
 * 10_functions.c - Function definitions, parameters, return types
 *
 * Functions encapsulate reusable code. In C, functions can:
 * - Take parameters by value or by pointer
 * - Return values, pointers, or void
 * - Be declared before use (prototypes)
 */

#include <stdio.h>
#include <string.h>

/* Function prototypes (declarations) - allows calling before definition */
int add(int a, int b);
void swap(int *a, int *b);
const char *get_day_name(int day);
int *find_max(int *arr, int size);

/* Simple function returning int */
int add(int a, int b) {
    return a + b;
}

/* Function returning void, modifying through pointers */
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

/* Function returning pointer to string literal */
const char *get_day_name(int day) {
    static const char *days[] = {
        "Sunday", "Monday", "Tuesday", "Wednesday",
        "Thursday", "Friday", "Saturday"
    };

    if (day < 0 || day > 6) {
        return "Invalid";
    }
    return days[day];
}

/* Function returning pointer to element in array */
int *find_max(int *arr, int size) {
    if (arr == NULL || size <= 0) {
        return NULL;
    }

    int *max = &arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > *max) {
            max = &arr[i];
        }
    }
    return max;
}

/* Function with array parameter (array decays to pointer) */
int sum_array(const int arr[], int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

/* Function with struct parameter (passed by value - copies) */
struct Point {
    int x;
    int y;
};

void print_point(struct Point p) {
    printf("Point(%d, %d)\n", p.x, p.y);
}

/* Function with struct pointer (no copy, can modify) */
void move_point(struct Point *p, int dx, int dy) {
    p->x += dx;
    p->y += dy;
}

/* Recursive function */
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

/* Static function - only visible in this file */
static int helper_function(int x) {
    return x * 2;
}

int main(void) {
    printf("=== Basic Function Call ===\n");
    int result = add(3, 4);
    printf("add(3, 4) = %d\n", result);

    printf("\n=== Pass by Pointer (Swap) ===\n");
    int x = 10, y = 20;
    printf("Before swap: x=%d, y=%d\n", x, y);
    swap(&x, &y);
    printf("After swap:  x=%d, y=%d\n", x, y);

    printf("\n=== Returning Pointer to String ===\n");
    printf("Day 0: %s\n", get_day_name(0));
    printf("Day 3: %s\n", get_day_name(3));
    printf("Day 9: %s\n", get_day_name(9));

    printf("\n=== Returning Pointer to Array Element ===\n");
    int numbers[] = { 5, 2, 9, 1, 7 };
    int *max_ptr = find_max(numbers, 5);
    if (max_ptr != NULL) {
        printf("Max value: %d\n", *max_ptr);
        printf("Max at index: %ld\n", max_ptr - numbers);
    }

    printf("\n=== Array as Parameter ===\n");
    printf("Sum of array: %d\n", sum_array(numbers, 5));

    printf("\n=== Struct Parameters ===\n");
    struct Point p = { 10, 20 };
    printf("Before: ");
    print_point(p);
    move_point(&p, 5, -3);
    printf("After move(5, -3): ");
    print_point(p);

    printf("\n=== Recursive Function ===\n");
    for (int i = 0; i <= 5; i++) {
        printf("factorial(%d) = %d\n", i, factorial(i));
    }

    printf("\n=== Static Helper Function ===\n");
    printf("helper_function(5) = %d\n", helper_function(5));

    return 0;
}
