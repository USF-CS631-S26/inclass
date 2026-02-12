/* scan.c - instrumented scanner with step-by-step tracing */

#include "ntlang.h"

char *scan_token_strings[] = SCAN_TOKEN_STRINGS;

/* Short name for token type (without TK_ prefix) */
static const char *scan_token_short(enum scan_token_enum id) {
    switch (id) {
        case TK_INTLIT: return "INTLIT";
        case TK_PLUS:   return "PLUS";
        case TK_MINUS:  return "MINUS";
        case TK_EOT:    return "EOT";
        case TK_ANY:    return "ANY";
    }
    return "???";
}

/* Print an ASCII art box diagram of the scan_table state */
static void scan_table_print_state(struct scan_table_st *st) {
    /* Show at least 4 columns, or len if larger */
    int cols = st->len;
    if (cols < 4) cols = 4;

    #define COL_W 15

    printf("\n  scan_table state (len=%d, cur=%d):\n", st->len, st->cur);

    /* Top border */
    printf("  +");
    for (int c = 0; c < cols; c++) {
        for (int k = 0; k < COL_W; k++) printf("-");
        printf("+");
    }
    printf("\n");

    /* Header row: indices */
    printf("  |");
    for (int c = 0; c < cols; c++) {
        char buf[COL_W + 1];
        snprintf(buf, sizeof(buf), "     [%d]", c);
        printf("%-*s|", COL_W, buf);
    }
    printf("\n");

    /* Content row: token type and value */
    printf("  |");
    for (int c = 0; c < cols; c++) {
        if (c < st->len) {
            char buf[COL_W + 1];
            snprintf(buf, sizeof(buf), " %s(\"%s\")",
                     scan_token_short(st->table[c].id),
                     st->table[c].value);
            /* Truncate if too long */
            if ((int)strlen(buf) > COL_W) buf[COL_W] = '\0';
            printf("%-*s|", COL_W, buf);
        } else {
            printf("%-*s|", COL_W, "");
        }
    }
    printf("\n");

    /* Bottom border */
    printf("  +");
    for (int c = 0; c < cols; c++) {
        for (int k = 0; k < COL_W; k++) printf("-");
        printf("+");
    }
    printf("\n");

    /* Cursor marker */
    printf("  ");
    /* Position the ^cur marker under the correct column */
    int offset = 1 + st->cur * (COL_W + 1) + 4;
    for (int k = 0; k < offset; k++) printf(" ");
    printf("^cur\n");

    #undef COL_W
}

void scan_table_init(struct scan_table_st *st) {
    st->len = 0;
    st->cur = 0;
    printf("scan_table_init(): len=0, cur=0\n");
}

void scan_token_print(struct scan_token_st *tp) {
    printf("%s(\"%s\")\n", scan_token_strings[tp->id], tp->value);
}

void scan_table_print(struct scan_table_st *st) {
    int i;

    for (i = 0; i < st->len; i++) {
        scan_token_print(&st->table[i]);
    }
}

struct scan_token_st * scan_table_new_token(struct scan_table_st *st) {
    struct scan_token_st *tp;
    int old_len = st->len;

    tp = &(st->table[st->len]);
    st->len += 1;

    printf("\n  ALLOC scan_table.table[%d]  (scan_table.len: %d -> %d)\n",
           old_len, old_len, st->len);

    return tp;
}

bool scan_is_whitespace(char ch) {
    return (ch == ' ') || (ch == '\t');
}

char *scan_whitespace(char *p, char *end) {
    while(scan_is_whitespace(*p) && (p < end)) {
        p += 1;
    }
    return p;
}

bool scan_is_digit(char ch) {
    return (ch >= '0' && ch <= '9');
}

char * scan_intlit(char *p, char *end, struct scan_token_st *tp) {
    int i = 0;

    while (scan_is_digit(*p) && (p < end)) {
        tp->value[i] = *p;
        p += 1;
        i += 1;
    }
    tp->value[i] = '\0';
    tp->id = TK_INTLIT;

    return p;
}

char * scan_token_helper(struct scan_token_st *tp, char *p, int len,
                       enum scan_token_enum id) {
    int i;

    tp->id = id;
    for (i = 0; i < len; i++) {
        tp->value[i] = *p;
        p += 1;
    }
    tp->value[i] = '\0';
    return p;
}

char * scan_token(char *p, char *end, struct scan_token_st *tp) {
    if (p == end) {
        printf("  scan_token(): p=\"\" -- end of input\n");
        tp->value[0] = '\0';
        tp->id = TK_EOT;
        printf("    scanned %s(\"\")\n", scan_token_strings[TK_EOT]);
    } else if (scan_is_whitespace(*p)) {
        printf("  scan_token(): p=\"%s\" -- whitespace, skipping\n", p);
        p = scan_whitespace(p, end);
        p = scan_token(p, end, tp);
    } else if (scan_is_digit(*p)) {
        printf("  scan_token(): p=\"%s\" -- digit found, scanning intlit\n", p);
        p = scan_intlit(p, end, tp);
        printf("    scanned %s(\"%s\")\n", scan_token_strings[tp->id], tp->value);
    } else if (*p == '+') {
        printf("  scan_token(): p=\"%s\" -- symbol '+'\n", p);
        p = scan_token_helper(tp, p, 1, TK_PLUS);
        printf("    scanned %s(\"%s\")\n", scan_token_strings[tp->id], tp->value);
    } else if (*p == '-') {
        printf("  scan_token(): p=\"%s\" -- symbol '-'\n", p);
        p = scan_token_helper(tp, p, 1, TK_MINUS);
        printf("    scanned %s(\"%s\")\n", scan_token_strings[tp->id], tp->value);
    } else {
        printf("scan error: invalid char: %c\n", *p);
        exit(-1);
    }
    return p;
}

void scan_table_scan(struct scan_table_st *st, char *input) {
    struct scan_token_st *tp;
    char *p = input;
    char *end;
    int len;

    len = strnlen(input, SCAN_INPUT_LEN);
    end = p + len;

    printf("scan_table_scan(): input=\"%s\", len=%d\n", input, len);

    do {
        tp = scan_table_new_token(st);
        p = scan_token(p, end, tp);
        scan_table_print_state(st);
        if (tp->id == TK_EOT) {
            break;
       }
    } while(true);

    printf("\nScan complete: %d tokens in scan_table\n", st->len);
}

struct scan_token_st * scan_table_get(struct scan_table_st *st, int i) {
    return &(st->table[st->cur + i]);
}

bool scan_table_accept(struct scan_table_st *st, enum scan_token_enum tk_expected) {
    struct scan_token_st *tp;

    if (tk_expected == TK_ANY) {
        st->cur += 1;
        return true;
    }

    tp = scan_table_get(st, 0);

    if (tp->id == tk_expected) {
        st->cur += 1;
        return true;
    }

    return false;
}
