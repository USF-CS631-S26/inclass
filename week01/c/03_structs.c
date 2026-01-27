/*
 * 03_structs.c - Struct definition and usage
 *
 * Structs group related data together into a single type.
 * They are the foundation of data organization in C.
 */

#include <stdio.h>
#include <string.h>

/* Basic struct definition */
struct Point {
    int x;
    int y;
};

/* Struct with different field types */
struct Person {
    char name[50];
    int age;
    double height;
};

/* Using typedef to avoid writing 'struct' everywhere */
typedef struct {
    char text[100];
    int line;
    int column;
} Token;

/* Nested structs */
struct Rectangle {
    struct Point top_left;
    struct Point bottom_right;
};

/* Function that takes struct by value (copies the data) */
void print_point(struct Point p) {
    printf("Point(%d, %d)\n", p.x, p.y);
}

/* Function that takes struct by pointer (no copy, can modify) */
void move_point(struct Point *p, int dx, int dy) {
    p->x += dx;  /* Arrow operator for pointer->member */
    p->y += dy;
}

int main(void) {
    /* Initialize struct with designated initializers */
    struct Point origin = { .x = 0, .y = 0 };

    /* Initialize struct in order */
    struct Point corner = { 10, 20 };

    /* Declare then assign fields */
    struct Person student;
    strncpy(student.name, "Alice", sizeof(student.name) - 1);
    student.name[sizeof(student.name) - 1] = '\0';
    student.age = 20;
    student.height = 1.65;

    /* Using typedef struct */
    Token tok = { .text = "hello", .line = 1, .column = 5 };

    /* Nested struct initialization */
    struct Rectangle rect = {
        .top_left = { 0, 0 },
        .bottom_right = { 100, 50 }
    };

    printf("Origin: ");
    print_point(origin);

    printf("Corner before move: ");
    print_point(corner);

    move_point(&corner, 5, -3);  /* Pass address with & */
    printf("Corner after move(5, -3): ");
    print_point(corner);

    printf("\nStudent: %s, age %d, height %.2fm\n",
           student.name, student.age, student.height);

    printf("\nToken: '%s' at line %d, column %d\n",
           tok.text, tok.line, tok.column);

    printf("\nRectangle from (%d,%d) to (%d,%d)\n",
           rect.top_left.x, rect.top_left.y,
           rect.bottom_right.x, rect.bottom_right.y);

    return 0;
}
