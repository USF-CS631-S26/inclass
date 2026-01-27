/*
 * 02_enums.c - Basic enumeration types
 *
 * Enums create named integer constants, making code more readable.
 * By default, values start at 0 and increment by 1.
 */

#include <stdio.h>

/* Basic enum - values are 0, 1, 2, 3 */
enum Direction {
    NORTH,  /* 0 */
    EAST,   /* 1 */
    SOUTH,  /* 2 */
    WEST    /* 3 */
};

/* Enum with explicit values */
enum TokenType {
    TOKEN_EOF = 0,
    TOKEN_PLUS = '+',      /* 43 (ASCII value) */
    TOKEN_MINUS = '-',     /* 45 */
    TOKEN_NUMBER = 256,    /* Start custom tokens above ASCII */
    TOKEN_IDENTIFIER = 257
};

/* Using typedef for cleaner syntax */
typedef enum {
    STATE_IDLE,
    STATE_RUNNING,
    STATE_PAUSED,
    STATE_STOPPED
} State;

/* Function using enum parameter */
const char *direction_name(enum Direction dir) {
    switch (dir) {
        case NORTH: return "North";
        case EAST:  return "East";
        case SOUTH: return "South";
        case WEST:  return "West";
        default:    return "Unknown";
    }
}

int main(void) {
    /* Declare and use enum variables */
    enum Direction heading = NORTH;
    State current_state = STATE_IDLE;

    printf("Direction values:\n");
    printf("  NORTH = %d\n", NORTH);
    printf("  EAST  = %d\n", EAST);
    printf("  SOUTH = %d\n", SOUTH);
    printf("  WEST  = %d\n", WEST);

    printf("\nToken values:\n");
    printf("  TOKEN_EOF    = %d\n", TOKEN_EOF);
    printf("  TOKEN_PLUS   = %d (ASCII '+')\n", TOKEN_PLUS);
    printf("  TOKEN_NUMBER = %d\n", TOKEN_NUMBER);

    printf("\nCurrent heading: %s\n", direction_name(heading));

    /* Enums are integers, so arithmetic works (but use carefully) */
    heading = (enum Direction)((heading + 1) % 4);
    printf("After turning right: %s\n", direction_name(heading));

    printf("\nState value: %d\n", current_state);

    return 0;
}
