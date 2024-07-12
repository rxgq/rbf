#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

typedef enum {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    NUMBER,
    BAD,
} TokenType;

typedef struct {
    TokenType type;
    char *value;
} Token;

int token_count = 0;
int current = -1;

void tokenize_number(Token* tokens, char* cptr) {
    char* digits = (char *)malloc(64 * sizeof(char));
    int number_length = 0;

    while (isdigit(cptr[current])) {
        digits[number_length] = cptr[current];
        number_length++;
        current++;
    }
    current--;

    digits[number_length] = '\0';
    tokens[token_count].value = digits;
}

void add_token() {
    
}

void next_token(char c, Token* tokens, char* cptr) {
    TokenType type;
    char *value = NULL;

    if (c == '+') {
        type = PLUS;
        value = (char *)malloc(2 * sizeof(char));
        value[0] = c;
        value[1] = '\0';
    } else if (c == '-') {
        type = MINUS;
        value = (char *)malloc(2 * sizeof(char));
        value[0] = c;
        value[1] = '\0';
    } else if (c == '*') {
        type = STAR;
        value = (char *)malloc(2 * sizeof(char));
        value[0] = c;
        value[1] = '\0';
    } else if (c == '/') {
        type = SLASH;
        value = (char *)malloc(2 * sizeof(char));
        value[0] = c;
        value[1] = '\0';
    } else if (isdigit(c)) {
        type = NUMBER;
        tokenize_number(tokens, cptr);
    } else {
        type = BAD;
        value = (char *)malloc(2 * sizeof(char));
        value[0] = ' ';
        value[1] = '\0';
    }

    tokens[token_count].type = type;
    if (type != NUMBER) {
        tokens[token_count].value = value;
    }

    token_count++;
}

Token* tokenize(char* cptr, int sz) {
    Token* tokens = (Token *)malloc(10 * sizeof(Token));

    while (current < sz - 1) {
        current += 1;
        char c = cptr[current];
        
        next_token(c, tokens, cptr);
    }

    return tokens;
}

void token_out(TokenType type, Token* tokens, int i) {
    switch (type) {
        case PLUS:
            printf("%d: PLUS (%s)\n", i, tokens[i].value);
            break;
        case MINUS:
            printf("%d: MINUS (%s)\n", i, tokens[i].value);
            break;
        case STAR:
            printf("%d: STAR (%s)\n", i, tokens[i].value);
            break;
        case SLASH:
            printf("%d: SLASH (%s)\n", i, tokens[i].value);
            break;
        case BAD:
            printf("%d: BAD (%s)\n", i, tokens[i].value);
            break;
        case NUMBER:
            printf("%d: NUMBER (%s)\n", i, tokens[i].value);
            break;
        default:
            printf("%d: UNKNOWN (%s)\n", i, tokens[i].value);
            break;
    }
}

int main(int argc, char *argv[]) {
    FILE *fptr = NULL;
    char* buff = NULL;

    if (argc < 2) {
        printf("Usage: ./main <path_to_your_code>\n");
        return 1;
    } 

    if ((fptr = fopen(argv[1], "r")) == NULL) {
        perror("error opening file");
        return 1;
    }

    fseek(fptr, 0, SEEK_END);
    int sz = ftell(fptr);
    fseek(fptr, 0, SEEK_SET);

    buff = (char *)malloc((sz + 1) * sizeof(char));
    fread(buff, sz, 1, fptr);
    buff[sz] = '\0';

    Token* tokens = tokenize(buff, sz);

    for (int i = 0; i < token_count; i++) {
        token_out(tokens[i].type, tokens, i);
    }
    
    fclose(fptr);
    free(buff);

    for (int i = 0; i < token_count; i++) {
        free(tokens[i].value);
    }
    free(tokens);

    return 0;
}
