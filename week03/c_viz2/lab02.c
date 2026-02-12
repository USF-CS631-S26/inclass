/* lab02.c - instrumented parsing visualization */

#include "ntlang.h"

int main(int argc, char **argv) {
    struct scan_table_st scan_table;
    struct parse_table_st parse_table;
    struct parse_node_st *parse_tree;
    char *input;

    if (argc != 2) {
        printf("Usage: lab02 <expression>\n");
        printf("  Example: lab02 \"1 + 2\"\n");
        exit(-1);
    }

    input = argv[1];

    /* Phase 1: Scanning */
    printf("=== Scanning \"%s\" ===\n", input);
    scan_table_init(&scan_table);
    scan_table_scan(&scan_table, input);

    /* Phase 2: Parsing */
    printf("\n=== Parsing ===\n");
    parse_table_init(&parse_table);
    parse_tree = parse_program(&parse_table, &scan_table);

    /* Phase 3: Print tree */
    printf("\n=== Parse Tree ===\n");
    parse_tree_print(parse_tree);

    return 0;
}
