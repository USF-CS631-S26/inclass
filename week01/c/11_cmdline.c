/*
 * 11_cmdline.c - argc, argv usage
 *
 * Command line arguments are passed to main() as:
 *   argc - argument count (number of arguments including program name)
 *   argv - argument vector (array of string pointers)
 *
 * argv[0] is always the program name.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void print_usage(const char *program_name) {
    printf("Usage: %s [options] <args...>\n", program_name);
    printf("Options:\n");
    printf("  -h, --help     Show this help message\n");
    printf("  -v, --verbose  Enable verbose output\n");
    printf("  -n <number>    Specify a number\n");
}

int main(int argc, char *argv[]) {
    printf("=== Command Line Arguments ===\n");
    printf("argc = %d (number of arguments)\n", argc);
    printf("\nAll arguments:\n");
    for (int i = 0; i < argc; i++) {
        printf("  argv[%d] = \"%s\"\n", i, argv[i]);
    }

    /* argv[argc] is always NULL */
    printf("\nargv[argc] = %p (always NULL)\n", (void *)argv[argc]);

    /* Simple argument processing */
    printf("\n=== Argument Processing ===\n");
    int verbose = 0;
    int number = -1;
    int positional_count = 0;

    for (int i = 1; i < argc; i++) {
        /* Check for help flag */
        if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            print_usage(argv[0]);
            return 0;
        }

        /* Check for verbose flag */
        if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--verbose") == 0) {
            verbose = 1;
            printf("Verbose mode enabled\n");
            continue;
        }

        /* Check for -n with argument */
        if (strcmp(argv[i], "-n") == 0) {
            if (i + 1 < argc) {
                i++;  /* Move to next argument */
                number = atoi(argv[i]);  /* Convert string to int */
                printf("Number set to: %d\n", number);
            } else {
                fprintf(stderr, "Error: -n requires an argument\n");
                return 1;
            }
            continue;
        }

        /* Check for unknown options */
        if (argv[i][0] == '-') {
            fprintf(stderr, "Unknown option: %s\n", argv[i]);
            continue;
        }

        /* Positional argument */
        positional_count++;
        printf("Positional argument %d: \"%s\"\n", positional_count, argv[i]);
    }

    /* Summary */
    printf("\n=== Summary ===\n");
    printf("Verbose: %s\n", verbose ? "yes" : "no");
    printf("Number: %d\n", number);
    printf("Positional arguments: %d\n", positional_count);

    /* Example: require at least one argument */
    if (argc < 2) {
        printf("\nTry running with arguments:\n");
        printf("  %s hello world\n", argv[0]);
        printf("  %s -v -n 42 file.txt\n", argv[0]);
        printf("  %s --help\n", argv[0]);
    }

    return 0;
}
