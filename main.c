#include <stdio.h>
#include <stdlib.h>

typedef enum {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    BAD,
} TokenType;

typedef struct {
    TokenType type;
    char *value;
} Token;

int token_count = 0;

void next_token(char c, Token** tokens_ptr) {
    TokenType type;
    char *value = (char *)malloc(2 * sizeof(char));

    if (c == '+') {
        type = PLUS;
        value[0] = c;
    } else if (c == '-') {
        type = MINUS;
        value[0] = c;
    } else if (c == '*') {
        type = STAR;
        value[0] = c;
    } else if (c == '/') {
        type = SLASH;
        value[0] = c;
    } else {
        type = BAD;
        value[0] = NULL;
    }

    value[1] = '\0';

    (*tokens_ptr)[token_count].type = type;
    (*tokens_ptr)[token_count].value = value; 

    token_count++;
}

Token* tokenize(char* cptr, int sz) {
    Token* tokens = (Token *)malloc(10 * sizeof(Token));

    int current = -1;
    while (current < sz - 1) {
        current += 1;
        char c = cptr[current];
        
        next_token(c, &tokens);
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
        default:
            printf("%d: UNKNOWN (%s)\n", i, tokens[i].value);
            break;
    }
}

int main(int argc, char *argv[]) {
    FILE *fptr = NULL;
    char* buff = NULL;

    if (argv[1] == NULL) {
        printf("Usage: ./main.exe <path_to_your_code>");
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

    return 0;
}