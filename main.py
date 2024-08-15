from lexer import Lexer


def main():
    source = "(atom 23)"

    lexer = Lexer(source)
    lexer.tokenize()

    lexer.print()


if __name__ == "__main__":
    main()
