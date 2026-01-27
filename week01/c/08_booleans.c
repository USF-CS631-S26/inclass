/*
 * 08_booleans.c - stdbool.h and bool type
 *
 * C99 introduced <stdbool.h> which provides:
 *   - bool type
 *   - true (1) and false (0) constants
 *
 * Before C99, integers were used: 0 = false, non-zero = true
 */

#include <stdio.h>
#include <stdbool.h>  /* Provides bool, true, false */

/* Function returning bool */
bool is_even(int n) {
    return n % 2 == 0;
}

/* Function returning bool with multiple conditions */
bool is_valid_age(int age) {
    return age >= 0 && age <= 150;
}

/* Function demonstrating bool parameter */
void print_status(const char *name, bool active) {
    printf("%s is %s\n", name, active ? "active" : "inactive");
}

int main(void) {
    /* Declare bool variables */
    bool flag = true;
    bool done = false;

    printf("=== Bool Basics ===\n");
    printf("flag = %d (true)\n", flag);
    printf("done = %d (false)\n", done);

    /* Bool in conditions */
    printf("\n=== Bool in Conditions ===\n");
    if (flag) {
        printf("flag is true\n");
    }
    if (!done) {
        printf("done is false\n");
    }

    /* Using bool functions */
    printf("\n=== Bool Functions ===\n");
    for (int i = 1; i <= 5; i++) {
        printf("%d is %s\n", i, is_even(i) ? "even" : "odd");
    }

    /* Integer to bool conversion */
    printf("\n=== Integer to Bool ===\n");
    int zero = 0;
    int positive = 42;
    int negative = -1;

    printf("(bool)0  = %d\n", (bool)zero);      /* false */
    printf("(bool)42 = %d\n", (bool)positive);  /* true */
    printf("(bool)-1 = %d\n", (bool)negative);  /* true (non-zero) */

    /* Comparison results are bool-like */
    printf("\n=== Comparison Results ===\n");
    printf("(5 > 3) = %d\n", 5 > 3);
    printf("(5 < 3) = %d\n", 5 < 3);
    printf("(5 == 5) = %d\n", 5 == 5);

    /* Boolean operators */
    printf("\n=== Boolean Operators ===\n");
    bool a = true;
    bool b = false;
    printf("a = true, b = false\n");
    printf("a && b = %d (AND)\n", a && b);
    printf("a || b = %d (OR)\n", a || b);
    printf("!a = %d (NOT)\n", !a);
    printf("!b = %d (NOT)\n", !b);

    /* Short-circuit evaluation */
    printf("\n=== Short-Circuit Evaluation ===\n");
    int x = 0;
    /* Second part not evaluated because first is false */
    if (false && (x = 1)) { }
    printf("After (false && (x=1)): x = %d\n", x);

    /* Second part not evaluated because first is true */
    if (true || (x = 2)) { }
    printf("After (true || (x=2)): x = %d\n", x);

    /* Using bool with print_status */
    printf("\n=== Function with Bool Parameter ===\n");
    print_status("Server", true);
    print_status("Debug mode", false);

    /* Validation example */
    printf("\n=== Validation Example ===\n");
    int ages[] = { -5, 0, 25, 150, 200 };
    for (size_t i = 0; i < sizeof(ages) / sizeof(ages[0]); i++) {
        printf("Age %d: %s\n", ages[i],
               is_valid_age(ages[i]) ? "valid" : "invalid");
    }

    return 0;
}
