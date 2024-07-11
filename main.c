#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    FILE *fptr = NULL;
    char* buff = NULL;

    if (argv[1] == NULL) {
        printf("Usage: ./main.exe <path_to_your_code>");
        return 1;
    } 

    if ((fptr = fopen(argv[1], "rb")) == NULL) {
        perror("error opening file");
        return 1;
    }

    fseek(fptr, 0, SEEK_END);
    int sz = ftell(fptr);
    fseek(fptr, 0, SEEK_SET);

    buff = (char *)malloc(sz * sizeof(char));
    fread(buff, sz, 1, fptr);
    buff[sz] = '\0';

    return 0;
}