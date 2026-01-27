/*
 * 09_control_flow.c - if/else, while, do-while, for, switch
 *
 * C provides several control flow constructs for conditional
 * execution and looping.
 */

#include <stdio.h>

int main(void) {
    /* ==================== IF/ELSE ==================== */
    printf("=== if/else ===\n");
    int score = 85;

    if (score >= 90) {
        printf("Grade: A\n");
    } else if (score >= 80) {
        printf("Grade: B\n");
    } else if (score >= 70) {
        printf("Grade: C\n");
    } else {
        printf("Grade: F\n");
    }

    /* Single-line if (braces optional but recommended) */
    int x = 10;
    if (x > 0)
        printf("x is positive\n");

    /* ==================== WHILE LOOP ==================== */
    printf("\n=== while loop ===\n");
    int count = 0;
    while (count < 5) {
        printf("count = %d\n", count);
        count++;
    }

    /* while with break */
    printf("\nwhile with break:\n");
    int n = 0;
    while (1) {  /* Infinite loop */
        if (n >= 3) {
            break;  /* Exit the loop */
        }
        printf("n = %d\n", n);
        n++;
    }

    /* while with continue */
    printf("\nwhile with continue (skip evens):\n");
    int i = 0;
    while (i < 6) {
        i++;
        if (i % 2 == 0) {
            continue;  /* Skip rest of loop body */
        }
        printf("i = %d\n", i);
    }

    /* ==================== DO-WHILE LOOP ==================== */
    printf("\n=== do-while loop ===\n");
    /* Executes at least once, then checks condition */
    int attempts = 0;
    do {
        printf("Attempt %d\n", attempts);
        attempts++;
    } while (attempts < 3);

    /* Useful for input validation (simulated) */
    int input = -1;
    int tries = 0;
    do {
        input = tries;  /* Simulate reading input */
        printf("Read input: %d\n", input);
        tries++;
    } while (input < 0 && tries < 3);

    /* ==================== FOR LOOP ==================== */
    printf("\n=== for loop ===\n");
    for (int j = 0; j < 5; j++) {
        printf("j = %d\n", j);
    }

    /* Countdown */
    printf("\nCountdown: ");
    for (int k = 5; k > 0; k--) {
        printf("%d ", k);
    }
    printf("Liftoff!\n");

    /* Multiple variables */
    printf("\nMultiple loop variables:\n");
    for (int a = 0, b = 10; a < b; a++, b--) {
        printf("a=%d, b=%d\n", a, b);
    }

    /* ==================== SWITCH ==================== */
    printf("\n=== switch statement ===\n");
    char grade = 'B';

    switch (grade) {
        case 'A':
            printf("Excellent!\n");
            break;
        case 'B':
            printf("Good job!\n");
            break;
        case 'C':
            printf("Passing.\n");
            break;
        case 'D':
        case 'F':  /* Fall-through: D and F share this code */
            printf("Needs improvement.\n");
            break;
        default:
            printf("Invalid grade.\n");
            break;
    }

    /* Switch with integer */
    printf("\nSwitch with integer:\n");
    int day = 3;
    switch (day) {
        case 1: printf("Monday\n"); break;
        case 2: printf("Tuesday\n"); break;
        case 3: printf("Wednesday\n"); break;
        case 4: printf("Thursday\n"); break;
        case 5: printf("Friday\n"); break;
        default: printf("Weekend\n"); break;
    }

    return 0;
}
