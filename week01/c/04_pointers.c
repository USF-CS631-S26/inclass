/*
 * 04_pointers.c - Pointer basics and dereferencing
 *
 * A pointer stores the memory address of another variable.
 * Key operators:
 *   &  - "address of" - gets a variable's address
 *   *  - "dereference" - accesses value at an address
 */

#include <stdio.h>

int main(void) {
    /* Regular variable */
    int value = 42;

    /* Pointer declaration: int* means "pointer to int" */
    int *ptr = &value;  /* ptr now holds the address of value */

    printf("=== Basic Pointer Operations ===\n");
    printf("value     = %d\n", value);
    printf("&value    = %p  (address of value)\n", (void *)&value);
    printf("ptr       = %p  (pointer holds this address)\n", (void *)ptr);
    printf("*ptr      = %d  (dereferenced: value at address)\n", *ptr);

    /* Modifying through pointer */
    printf("\n=== Modifying Through Pointer ===\n");
    *ptr = 100;  /* Changes value through the pointer */
    printf("After *ptr = 100:\n");
    printf("value = %d\n", value);
    printf("*ptr  = %d\n", *ptr);

    /* Multiple pointers to same location */
    printf("\n=== Multiple Pointers ===\n");
    int *another_ptr = &value;
    printf("another_ptr points to same location: %d\n", *another_ptr);

    /* Pointer to pointer */
    printf("\n=== Pointer to Pointer ===\n");
    int **ptr_to_ptr = &ptr;
    printf("ptr_to_ptr  = %p  (address of ptr)\n", (void *)ptr_to_ptr);
    printf("*ptr_to_ptr = %p  (value of ptr)\n", (void *)*ptr_to_ptr);
    printf("**ptr_to_ptr = %d (value at ptr's address)\n", **ptr_to_ptr);

    /* NULL pointer */
    printf("\n=== NULL Pointer ===\n");
    int *null_ptr = NULL;
    printf("null_ptr = %p\n", (void *)null_ptr);
    if (null_ptr == NULL) {
        printf("Pointer is NULL - do not dereference!\n");
    }

    /* Pointer to different types have same size (address size) */
    printf("\n=== Pointer Sizes ===\n");
    printf("sizeof(int*)    = %zu bytes\n", sizeof(int *));
    printf("sizeof(char*)   = %zu bytes\n", sizeof(char *));
    printf("sizeof(double*) = %zu bytes\n", sizeof(double *));

    return 0;
}
