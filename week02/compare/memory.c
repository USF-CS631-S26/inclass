/*
 * memory.c - Demonstrating stack vs heap allocation in C
 *
 * Key concepts:
 * - Stack: automatic storage, fast, limited size, scoped lifetime
 * - Heap: manual management, slower, large capacity, explicit lifetime
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ============================================================
 * SECTION 1: Stack Allocation
 * ============================================================
 * Stack-allocated data lives in the function's stack frame.
 * It is automatically deallocated when the function returns.
 */

void stack_example(void) {
    /* Stack-allocated array - fixed size, known at compile time */
    char greeting[32] = "Hello, World!";

    printf("Stack string: %s\n", greeting);
    printf("  Address: %p\n", (void *)greeting);
    printf("  sizeof:  %zu bytes (full buffer)\n", sizeof(greeting));
    printf("  strlen:  %zu bytes (actual content)\n\n", strlen(greeting));

    /* We can modify it within bounds */
    strcpy(greeting, "Modified!");
    printf("After modification: %s\n\n", greeting);

    /* greeting is automatically freed when this function returns */
}

/* ============================================================
 * SECTION 2: The Dangling Pointer Problem
 * ============================================================
 * DANGER: Returning a pointer to stack memory is undefined behavior!
 * The memory is deallocated when the function returns.
 */

/* WARNING: This function has a bug - DO NOT do this! */
char *dangerous_stack_return(void) {
    char local_buffer[64] = "I live on the stack";

    /* BUG: returning pointer to local variable! */
    /* The memory will be invalid after this function returns */
    return local_buffer;  /* Compiler may warn about this */
}

void demonstrate_dangling_pointer(void) {
    printf("=== Dangling Pointer Demonstration ===\n");

    char *dangerous = dangerous_stack_return();

    /* This is UNDEFINED BEHAVIOR - the memory may be:
     * - Overwritten by subsequent function calls
     * - Appear to "work" sometimes (making bugs hard to find)
     * - Crash the program
     * - Contain garbage data
     */
    printf("Dangerous pointer contains: %s\n", dangerous);
    printf("(This may appear to work, crash, or show garbage)\n\n");
}

/* ============================================================
 * SECTION 3: Heap Allocation
 * ============================================================
 * Heap memory persists until explicitly freed.
 * The programmer is responsible for:
 * - Allocating (malloc, calloc, realloc)
 * - Freeing (free)
 * - Not using after free
 * - Not freeing twice
 */

char *safe_heap_return(const char *input) {
    /* Allocate on heap - memory persists after function returns */
    size_t len = strlen(input) + 1;  /* +1 for null terminator */
    char *heap_string = malloc(len);

    if (heap_string == NULL) {
        fprintf(stderr, "malloc failed!\n");
        return NULL;
    }

    strcpy(heap_string, input);

    /* Safe to return - caller owns this memory now */
    /* But who frees it? The caller must remember to! */
    return heap_string;
}

void heap_example(void) {
    printf("=== Heap Allocation Example ===\n");

    /* Caller receives ownership of heap memory */
    char *my_string = safe_heap_return("Hello from the heap!");

    if (my_string != NULL) {
        printf("Heap string: %s\n", my_string);
        printf("  Address: %p\n\n", (void *)my_string);

        /* We can resize heap allocations */
        my_string = realloc(my_string, 100);
        strcat(my_string, " Plus more text!");
        printf("After realloc: %s\n\n", my_string);

        /* CRITICAL: We must free the memory */
        free(my_string);

        /* After free, my_string is a "dangling pointer" */
        /* Using it is undefined behavior */
        /* Good practice: set to NULL after free */
        my_string = NULL;
    }
}

/* ============================================================
 * SECTION 4: Common Memory Bugs in C
 * ============================================================
 */

void memory_leak_example(void) {
    printf("=== Memory Leak Example ===\n");

    char *leaked = malloc(100);
    strcpy(leaked, "I will be leaked!");

    /* Oops - we return without freeing! */
    /* This memory is now unreachable and cannot be reclaimed */
    /* In long-running programs, leaks accumulate and exhaust memory */

    printf("Allocated but never freed: %s\n", leaked);
    printf("(Memory leak - no way to recover this memory)\n\n");

    /* Fix: free(leaked); */
}

void double_free_example(void) {
    printf("=== Double Free Example (COMMENTED OUT - would crash) ===\n");

    char *data = malloc(50);
    strcpy(data, "Free me once");

    free(data);

    /* DANGER: Uncommenting this would cause undefined behavior!
     * Double-free can:
     * - Crash the program
     * - Corrupt the heap
     * - Create security vulnerabilities
     */
    /* free(data); */  /* DON'T DO THIS! */

    printf("First free: OK\n");
    printf("Second free: Would crash or corrupt memory!\n\n");
}

void use_after_free_example(void) {
    printf("=== Use After Free Example ===\n");

    char *data = malloc(50);
    strcpy(data, "Valid data");
    printf("Before free: %s\n", data);

    free(data);

    /* DANGER: data now points to freed memory!
     * Reading or writing is undefined behavior.
     * May appear to work, making bugs hard to find.
     */

    /* This is a bug - the memory has been freed! */
    /* printf("After free: %s\n", data); */  /* DON'T DO THIS! */

    printf("After free: (accessing would be undefined behavior)\n\n");
}

/* ============================================================
 * SECTION 5: Passing Strings to Functions
 * ============================================================
 * Who owns what? C doesn't enforce this - it's all by convention.
 */

/* This function borrows the string - does not modify or free */
void print_string(const char *str) {
    printf("Borrowed string: %s\n", str);
    /* We don't free str - we're just borrowing it */
}

/* This function takes ownership - caller should not use after */
void take_ownership(char *str) {
    printf("Taking ownership of: %s\n", str);
    /* We own it now, we must free it */
    free(str);
}

/* This function modifies the borrowed string */
void modify_string(char *str, size_t buffer_size) {
    /* We modify but don't own - caller still responsible for memory */
    strncat(str, " [modified]", buffer_size - strlen(str) - 1);
}

void ownership_confusion_demo(void) {
    printf("=== Ownership Confusion Demo ===\n");

    char *heap_str = safe_heap_return("Heap string");

    /* Borrowing is OK */
    print_string(heap_str);

    /* After this call, heap_str is freed but still points to memory! */
    take_ownership(heap_str);

    /* BUG: heap_str is now dangling - don't use it! */
    /* print_string(heap_str); */  /* Would be undefined behavior! */

    printf("(heap_str is now invalid - using it would be a bug)\n\n");
}

/* ============================================================
 * MAIN
 * ============================================================
 */

int main(void) {
    printf("========================================\n");
    printf("C Memory Management Demonstration\n");
    printf("========================================\n\n");

    printf("=== Stack Allocation ===\n");
    stack_example();

    printf("\n");
    demonstrate_dangling_pointer();

    heap_example();

    memory_leak_example();

    double_free_example();

    use_after_free_example();

    ownership_confusion_demo();

    printf("========================================\n");
    printf("Summary of C Memory Pitfalls:\n");
    printf("========================================\n");
    printf("1. Dangling pointers (returning stack memory)\n");
    printf("2. Memory leaks (forgetting to free)\n");
    printf("3. Double free (freeing twice)\n");
    printf("4. Use after free (accessing freed memory)\n");
    printf("5. Ownership confusion (who frees what?)\n");
    printf("6. Buffer overflows (writing past allocation)\n");
    printf("\nRust prevents ALL of these at compile time!\n");

    return 0;
}
