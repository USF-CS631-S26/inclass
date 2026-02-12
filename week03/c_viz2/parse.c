/* parse.c - instrumented parsing with step-by-step tracing */

#include "ntlang.h"

static int trace_depth = 0;

static void trace_indent(void) {
    for (int i = 0; i < trace_depth * 2; i++) {
        printf(" ");
    }
}

/* Get the pool index of a node via pointer arithmetic */
static int parse_node_index(struct parse_table_st *pt, struct parse_node_st *np) {
    return (int)(np - pt->table);
}

extern char *scan_token_strings[];

/* Short name for token type (without TK_ prefix) for trace output */
static const char *tk_short(enum scan_token_enum id) {
    switch (id) {
        case TK_INTLIT: return "TK_INTLIT";
        case TK_PLUS:   return "TK_PLUS";
        case TK_MINUS:  return "TK_MINUS";
        case TK_EOT:    return "TK_EOT";
        case TK_ANY:    return "TK_ANY";
    }
    return "TK_???";
}

/* Short name for expression type */
static const char *expr_type_short(enum parse_expr_enum type) {
    switch (type) {
        case EX_INTVAL: return "INTVAL";
        case EX_OPER1:  return "OPER1";
        case EX_OPER2:  return "OPER2";
    }
    return "???";
}

static const char *oper_short(enum parse_oper_enum op) {
    switch (op) {
        case OP_PLUS:  return "PLUS";
        case OP_MINUS: return "MINUS";
        case OP_MULT:  return "MULT";
        case OP_DIV:   return "DIV";
    }
    return "???";
}

/* Recursively print an ASCII art tree rooted at np */
static void trace_print_tree_rec(struct parse_table_st *pt,
                                  struct parse_node_st *np,
                                  const char *prefix, bool is_last, bool is_root) {
    int idx = parse_node_index(pt, np);

    trace_indent();
    if (!is_root) {
        printf("%s%s", prefix, is_last ? "`-- " : "+-- ");
    }

    if (np->type == EX_INTVAL) {
        printf("[%d] %d\n", idx, np->intval.value);
    } else if (np->type == EX_OPER2) {
        printf("[%d] %s\n", idx, oper_short(np->oper2.oper));

        char child_prefix[256];
        if (is_root) {
            child_prefix[0] = '\0';
        } else {
            snprintf(child_prefix, sizeof(child_prefix), "%s%s",
                     prefix, is_last ? "    " : "|   ");
        }

        bool has_right = (np->oper2.right != NULL);
        if (np->oper2.left) {
            trace_print_tree_rec(pt, np->oper2.left, child_prefix, !has_right, false);
        }
        if (has_right) {
            trace_print_tree_rec(pt, np->oper2.right, child_prefix, true, false);
        }
    }
}

static void trace_print_tree(struct parse_table_st *pt, struct parse_node_st *np) {
    printf("\n");
    trace_indent();
    printf("current tree:\n");
    trace_print_tree_rec(pt, np, "", true, true);
}

/* Print an ASCII art box diagram of the parse_table state */
static void parse_table_print_state(struct parse_table_st *pt) {
    int cols = pt->len;
    if (cols < 4) cols = 4;

    #define PT_COL_W 18

    printf("\n");
    trace_indent();
    printf("parse_table state (len=%d):\n", pt->len);

    /* Top border */
    trace_indent();
    printf("+");
    for (int c = 0; c < cols; c++) {
        for (int k = 0; k < PT_COL_W; k++) printf("-");
        printf("+");
    }
    printf("\n");

    /* Row 1: indices */
    trace_indent();
    printf("|");
    for (int c = 0; c < cols; c++) {
        char buf[PT_COL_W + 1];
        snprintf(buf, sizeof(buf), "       [%d]", c);
        printf("%-*s|", PT_COL_W, buf);
    }
    printf("\n");

    /* Row 2: type */
    trace_indent();
    printf("|");
    for (int c = 0; c < cols; c++) {
        if (c < pt->len) {
            char buf[PT_COL_W + 1];
            snprintf(buf, sizeof(buf), " %s", expr_type_short(pt->table[c].type));
            printf("%-*s|", PT_COL_W, buf);
        } else {
            printf("%-*s|", PT_COL_W, "");
        }
    }
    printf("\n");

    /* Row 3: primary field (val=N or op=OPER) */
    trace_indent();
    printf("|");
    for (int c = 0; c < cols; c++) {
        if (c < pt->len) {
            char buf[PT_COL_W + 1];
            if (pt->table[c].type == EX_INTVAL) {
                snprintf(buf, sizeof(buf), " val=%d", pt->table[c].intval.value);
            } else if (pt->table[c].type == EX_OPER2) {
                snprintf(buf, sizeof(buf), " op=%s", oper_short(pt->table[c].oper2.oper));
            } else {
                buf[0] = '\0';
            }
            if ((int)strlen(buf) > PT_COL_W) buf[PT_COL_W] = '\0';
            printf("%-*s|", PT_COL_W, buf);
        } else {
            printf("%-*s|", PT_COL_W, "");
        }
    }
    printf("\n");

    /* Row 4: secondary field (L=[i] R=[j] for OPER2) */
    trace_indent();
    printf("|");
    for (int c = 0; c < cols; c++) {
        if (c < pt->len && pt->table[c].type == EX_OPER2) {
            char buf[PT_COL_W + 1];
            char lbuf[8], rbuf[8];
            if (pt->table[c].oper2.left)
                snprintf(lbuf, sizeof(lbuf), "[%d]", parse_node_index(pt, pt->table[c].oper2.left));
            else
                snprintf(lbuf, sizeof(lbuf), "?");
            if (pt->table[c].oper2.right)
                snprintf(rbuf, sizeof(rbuf), "[%d]", parse_node_index(pt, pt->table[c].oper2.right));
            else
                snprintf(rbuf, sizeof(rbuf), "?");
            snprintf(buf, sizeof(buf), " L=%s R=%s", lbuf, rbuf);
            if ((int)strlen(buf) > PT_COL_W) buf[PT_COL_W] = '\0';
            printf("%-*s|", PT_COL_W, buf);
        } else {
            printf("%-*s|", PT_COL_W, "");
        }
    }
    printf("\n");

    /* Bottom border */
    trace_indent();
    printf("+");
    for (int c = 0; c < cols; c++) {
        for (int k = 0; k < PT_COL_W; k++) printf("-");
        printf("+");
    }
    printf("\n");

    #undef PT_COL_W
}

void parse_table_init(struct parse_table_st *pt) {
    pt->len = 0;
}

struct parse_node_st * parse_node_new(struct parse_table_st *pt) {
    struct parse_node_st *np;
    int idx = pt->len;

    np = &(pt->table[pt->len]);
    memset(np, 0, sizeof(*np));
    pt->len += 1;

    trace_indent();
    printf("ALLOC node[%d] from parse_table.table[%d]\n", idx, idx);

    return np;
}

void parse_error(char *err) {
    printf("parse_error: %s\n", err);
    exit(-1);
}

char *parse_oper_strings[] = {"PLUS", "MINUS", "MULT", "DIV"};

struct parse_node_st * parse_program(struct parse_table_st *pt,
                                        struct scan_table_st *st);
struct parse_node_st * parse_expression(struct parse_table_st *pt,
                                        struct scan_table_st *st);
struct parse_node_st * parse_operand(struct parse_table_st *pt,
                                        struct scan_table_st *st);

struct parse_node_st * parse_program(struct parse_table_st *pt,
                                        struct scan_table_st *st) {
    struct parse_node_st *np1;
    struct scan_token_st *tp;

    tp = scan_table_get(st, 0);
    trace_indent();
    printf("ENTER parse_program  [cur=%d: %s(\"%s\")]\n",
           st->cur, tk_short(tp->id), tp->value);
    trace_depth++;

    np1 = parse_expression(pt, st);

    int old_cur = st->cur;
    if (!scan_table_accept(st, TK_EOT)) {
        parse_error("Expecting EOT");
    }
    trace_indent();
    printf("accept TK_EOT => YES (cur: %d -> %d)\n", old_cur, st->cur);

    trace_depth--;
    trace_indent();
    printf("EXIT parse_program => node[%d]\n", parse_node_index(pt, np1));

    return np1;
}

struct parse_node_st * parse_expression(struct parse_table_st *pt,
                                        struct scan_table_st *st) {
    struct scan_token_st *tp;
    struct parse_node_st *np1, *np2;

    tp = scan_table_get(st, 0);
    trace_indent();
    printf("ENTER parse_expression  [cur=%d: %s(\"%s\")]\n",
           st->cur, tk_short(tp->id), tp->value);
    trace_depth++;

    np1 = parse_operand(pt, st);

    while (true) {
        tp = scan_table_get(st, 0);
        trace_indent();
        printf("loop: peek cur=%d => %s(\"%s\")",
               st->cur, tk_short(tp->id), tp->value);

        if (tp->id == TK_PLUS || tp->id == TK_MINUS) {
            printf(" -- is operator, continue\n");

            int old_cur = st->cur;
            scan_table_accept(st, TK_ANY);
            trace_indent();
            printf("accept TK_ANY => consumed \"%s\" (cur: %d -> %d)\n",
                   tp->value, old_cur, st->cur);

            np2 = parse_node_new(pt);
            int np2_idx = parse_node_index(pt, np2);
            np2->type = EX_OPER2;
            trace_indent();
            printf("  node[%d].type = EX_OPER2\n", np2_idx);

            if (tp->id == TK_PLUS) {
                np2->oper2.oper = OP_PLUS;
                trace_indent();
                printf("  node[%d].oper2.oper = OP_PLUS\n", np2_idx);
            } else {
                np2->oper2.oper = OP_MINUS;
                trace_indent();
                printf("  node[%d].oper2.oper = OP_MINUS\n", np2_idx);
            }

            np2->oper2.left = np1;
            trace_indent();
            printf("  node[%d].oper2.left = node[%d]\n",
                   np2_idx, parse_node_index(pt, np1));

            np2->oper2.right = parse_operand(pt, st);
            trace_indent();
            printf("  node[%d].oper2.right = node[%d]\n",
                   np2_idx, parse_node_index(pt, np2->oper2.right));
            parse_table_print_state(pt);
            trace_print_tree(pt, np2);

            trace_indent();
            printf("np1 = node[%d]  (left-associative: this becomes the new left subtree)\n",
                   np2_idx);
            np1 = np2;
        } else {
            printf(" -- not operator, break\n");
            break;
        }
    }

    trace_depth--;
    trace_indent();
    printf("EXIT parse_expression => node[%d]\n", parse_node_index(pt, np1));

    return np1;
}

struct parse_node_st * parse_operand(struct parse_table_st *pt,
                                     struct scan_table_st *st) {
    struct scan_token_st *tp;
    struct parse_node_st *np1;

    tp = scan_table_get(st, 0);
    trace_indent();
    printf("ENTER parse_operand  [cur=%d: %s(\"%s\")]\n",
           st->cur, tk_short(tp->id), tp->value);
    trace_depth++;

    int old_cur = st->cur;
    if (scan_table_accept(st, TK_INTLIT)) {
        trace_indent();
        printf("accept TK_INTLIT => YES, consumed \"%s\" (cur: %d -> %d)\n",
               tp->value, old_cur, st->cur);

        tp = scan_table_get(st, -1);
        np1 = parse_node_new(pt);
        int idx = parse_node_index(pt, np1);
        np1->type = EX_INTVAL;
        trace_indent();
        printf("  node[%d].type = EX_INTVAL\n", idx);
        np1->intval.value = atoi(tp->value);
        trace_indent();
        printf("  node[%d].intval.value = %d\n", idx, np1->intval.value);
        parse_table_print_state(pt);
        trace_print_tree(pt, np1);
    } else {
        parse_error("Bad operand");
    }

    trace_depth--;
    trace_indent();
    printf("EXIT parse_operand => node[%d] (INTVAL %d)\n",
           parse_node_index(pt, np1), np1->intval.value);

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
