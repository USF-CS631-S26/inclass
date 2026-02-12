/* parse.c - parsing and parse tree construction */
/* Instrumented version for visualizing parser behavior */

#include "ntlang.h"

/* Global step counter for instrumentation output */
static int viz_step = 0;

void parse_table_init(struct parse_table_st *pt) {
    pt->len = 0;
}

char *parse_oper_strings[] = {"PLUS", "MINUS", "MULT", "DIV"};
char *parse_expr_strings[] = {"EX_INTVAL", "EX_OPER1", "EX_OPER2"};

/*
 * Instrumentation: dump the full parse_table contents
 */
void viz_parse_table_dump(struct parse_table_st *pt, const char *label) {
    printf("  parse_table (%s) [len=%d]:\n", label, pt->len);
    if (pt->len == 0) {
        printf("    (empty)\n");
        return;
    }
    printf("    +-------+------------+-----------+--------------------+\n");
    printf("    | index | type       | value/op  | children (indices) |\n");
    printf("    +-------+------------+-----------+--------------------+\n");
    for (int i = 0; i < pt->len; i++) {
        struct parse_node_st *np = &pt->table[i];
        if (np->type == EX_INTVAL) {
            printf("    | %5d | %-10s | %-9d |                    |\n",
                   i, parse_expr_strings[np->type], np->intval.value);
        } else if (np->type == EX_OPER2) {
            /* Compute child indices by pointer arithmetic */
            char left_str[8] = "---";
            char right_str[8] = "---";
            if (np->oper2.left) {
                snprintf(left_str, sizeof(left_str), "%d",
                         (int)(np->oper2.left - pt->table));
            }
            if (np->oper2.right) {
                snprintf(right_str, sizeof(right_str), "%d",
                         (int)(np->oper2.right - pt->table));
            }
            printf("    | %5d | %-10s | %-9s | left=%s, right=%s  |\n",
                   i, parse_expr_strings[np->type],
                   parse_oper_strings[np->oper2.oper],
                   left_str, right_str);
        } else if (np->type == EX_OPER1) {
            int operand_idx = (int)(np->oper1.operand - pt->table);
            printf("    | %5d | %-10s | %-9s | operand=%d          |\n",
                   i, parse_expr_strings[np->type],
                   parse_oper_strings[np->oper1.oper],
                   operand_idx);
        }
    }
    printf("    +-------+------------+-----------+--------------------+\n");
}

/*
 * Instrumentation: print the parse tree rooted at a given node
 */
static void viz_tree_print_expr(struct parse_table_st *pt,
                                struct parse_node_st *np, int level) {
    int idx = (int)(np - pt->table);

    /* indent */
    for (int i = 0; i < level * 4; i++) printf(" ");

    if (np->type == EX_INTVAL) {
        printf("[%d] INTVAL %d\n", idx, np->intval.value);
    } else if (np->type == EX_OPER2) {
        printf("[%d] OPER2 %s\n", idx, parse_oper_strings[np->oper2.oper]);
        viz_tree_print_expr(pt, np->oper2.left, level + 1);
        viz_tree_print_expr(pt, np->oper2.right, level + 1);
    } else if (np->type == EX_OPER1) {
        printf("[%d] OPER1 %s\n", idx, parse_oper_strings[np->oper1.oper]);
        viz_tree_print_expr(pt, np->oper1.operand, level + 1);
    }
}

void viz_parse_tree_snapshot(struct parse_table_st *pt,
                             struct parse_node_st *root,
                             const char *label) {
    printf("  parse tree (%s):\n", label);
    if (root == NULL) {
        printf("    (empty)\n");
        return;
    }
    printf("    ");
    /* Print tree with extra leading spaces for alignment */
    viz_tree_print_expr(pt, root, 0);
}

/*
 * Instrumented parse_node_new: shows allocation event
 */
struct parse_node_st * parse_node_new(struct parse_table_st *pt) {
    struct parse_node_st *np;
    int idx = pt->len;

    np = &(pt->table[pt->len]);
    memset(np, 0, sizeof(*np));
    pt->len += 1;

    viz_step++;
    printf("==== Step %d: parse_node_new() => table[%d] allocated ====\n",
           viz_step, idx);

    return np;
}

void parse_error(char *err) {
    printf("parse_error: %s\n", err);
    exit(-1);
}


/* Prototypes */
struct parse_node_st * parse_program(struct parse_table_st *pt,
                                        struct scan_table_st *st);
struct parse_node_st * parse_expression(struct parse_table_st *pt,
                                        struct scan_table_st *st);
struct parse_node_st * parse_operand(struct parse_table_st *pt,
                                        struct scan_table_st *st);

struct parse_node_st * parse_program(struct parse_table_st *pt,
                                        struct scan_table_st *st) {
    struct parse_node_st *np1;

    printf("\n>>> parse_program: begin\n");

    np1 = parse_expression(pt, st);

    if (!scan_table_accept(st, TK_EOT)) {
        parse_error("Expecting EOT");
    }

    printf("\n>>> parse_program: complete\n");

    return np1;
}

struct parse_node_st * parse_expression(struct parse_table_st *pt,
                                        struct scan_table_st *st) {
    extern char *scan_token_strings[];
    struct scan_token_st *tp;
    struct parse_node_st *np1, *np2;

    printf("\n>>> parse_expression: parsing first operand\n");
    np1 = parse_operand(pt, st);

    while (true) {
        tp = scan_table_get(st, 0);
        printf("\n>>> parse_expression: next token is %s(\"%s\")\n",
               scan_token_strings[tp->id], tp->value);

        if (tp->id == TK_PLUS) {
            scan_table_accept(st, TK_ANY);
            printf(">>> parse_expression: accepted '+', allocating OPER2 node\n");

            np2 = parse_node_new(pt);
            np2->type = EX_OPER2;
            np2->oper2.oper = OP_PLUS;
            np2->oper2.left = np1;

            printf("  node[%d].type = EX_OPER2, oper = PLUS, left = node[%d]\n",
                   (int)(np2 - pt->table), (int)(np1 - pt->table));

            viz_parse_table_dump(pt, "after OPER2 allocated, right not yet set");

            printf("\n>>> parse_expression: parsing right operand\n");
            np2->oper2.right = parse_operand(pt, st);

            printf("  node[%d].right = node[%d]\n",
                   (int)(np2 - pt->table),
                   (int)(np2->oper2.right - pt->table));

            viz_parse_table_dump(pt, "after OPER2 complete");
            viz_parse_tree_snapshot(pt, np2, "current subtree");

            np1 = np2;
        } else {
            printf(">>> parse_expression: no more operators, done\n");
            break;
        }
    }

    return np1;
}

struct parse_node_st * parse_operand(struct parse_table_st *pt,
                                     struct scan_table_st *st) {
    struct scan_token_st *tp;
    struct parse_node_st *np1;

    if (scan_table_accept(st, TK_INTLIT)) {
        tp = scan_table_get(st, -1);
        printf(">>> parse_operand: accepted TK_INTLIT(\"%s\")\n", tp->value);

        np1 = parse_node_new(pt);
        np1->type = EX_INTVAL;
        np1->intval.value = atoi(tp->value);

        printf("  node[%d].type = EX_INTVAL, value = %d\n",
               (int)(np1 - pt->table), np1->intval.value);

        viz_parse_table_dump(pt, "after INTVAL");
        viz_parse_tree_snapshot(pt, np1, "current leaf");
    } else {
        parse_error("Bad operand");
    }

    return np1;
}

void parse_tree_print_indent(int level) {
    level *= 2;
    for (int i = 0; i < level; i++) {
        printf(".");
    }
}

void parse_tree_print_expr(struct parse_node_st *np, int level) {
    parse_tree_print_indent(level);
    printf("EXPR ");

    if (np->type == EX_INTVAL) {
        printf("INTVAL %d\n", np->intval.value);
    } else if (np->type == EX_OPER2) {
        printf("OPER2 %s\n", parse_oper_strings[np->oper2.oper]);
        parse_tree_print_expr(np->oper2.left, level+1);
        parse_tree_print_expr(np->oper2.right, level+1);
    }
}

void parse_tree_print(struct parse_node_st *np) {
    parse_tree_print_expr(np, 0);
}
