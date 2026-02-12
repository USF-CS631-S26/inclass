/* lab02.c - instrumented parsing implementation */
/* Shows how nodes are allocated, parse_table state, and parse tree growth */

#include "ntlang.h"

int main(int argc, char **argv) {
    struct scan_table_st scan_table;
    struct parse_table_st parse_table;
    struct parse_node_st *parse_tree;
    char *input;

    if (argc != 2) {
        printf("Usage: lab02 <expression>\n");
        printf("  Example: lab02 \"1 + 2\"\n");
        printf("  Example: lab02 \"1 + 2 + 3\"\n");
        printf("  Example: lab02 \"10 + 20 + 30 + 40\"\n");
        exit(-1);
    }

    input = argv[1];

    /* Scan */
    scan_table_init(&scan_table);
    scan_table_scan(&scan_table, input);

    printf("========================================\n");
    printf("Input: \"%s\"\n", input);
    printf("========================================\n");
    printf("\nTokens:\n");
    scan_table_print(&scan_table);

    printf("\n========================================\n");
    printf("Begin instrumented parse\n");
    printf("========================================\n");

    /* Parse with instrumentation */
    parse_table_init(&parse_table);
    parse_tree = parse_program(&parse_table, &scan_table);

    printf("\n========================================\n");
    printf("Final parse_table state\n");
    printf("========================================\n");
    viz_parse_table_dump(&parse_table, "final");

    printf("\n========================================\n");
    printf("Final parse tree\n");
    printf("========================================\n");
    parse_tree_print(parse_tree);

    return 0;
}
