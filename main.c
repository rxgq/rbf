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
    char value;
} Token;

int token_count = 0;

void next_token(char c, Token** tokens_ptr) {
    TokenType type;

    if (c == '+') {
        type = PLUS;
    } else if (c == '-') {
        type = MINUS;
    } else if (c == '*') {
        type = STAR;
    } else if (c == '/') {
        type = SLASH;
    } else {
        type = BAD;
    }

    if (token_count >= 10) {
        *tokens_ptr = realloc(*tokens_ptr, (token_count + 10) * sizeof(Token));
        if (!(*tokens_ptr)) {
            perror("realloc failed");
            exit(1);
        }
    }

    (*tokens_ptr)[token_count].type = type;
    (*tokens_ptr)[token_count].value = c; 

    token_count++;
}

Token* tokenize(char* cptr, int sz) {
    Token* tokens = (Token *)malloc(10 * sizeof(Token));

    int current = -1;
    while (current < sz) {
        current += 1;
        char c = cptr[current];
        
        next_token(c, &tokens);
    }

    return tokens;
}

void token_out(TokenType type, Token* tokens, int i) {
    switch (type) {
        case PLUS:
            printf("%d: PLUS (%c)\n", i, tokens[i].value);
            break;
        case MINUS:
            printf("%d: MINUS (%c)\n", i, tokens[i].value);
            break;
        case STAR:
            printf("%d: STAR (%c)\n", i, tokens[i].value);
            break;
        case SLASH:
            printf("%d: SLASH (%c)\n", i, tokens[i].value);
            break;
        case BAD:
            printf("%d: BAD (%c)\n", i, tokens[i].value);
            break;
        default:
            printf("%d: UNKNOWN (%c)\n", i, tokens[i].value);
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