from enum import Enum


class TokenType(Enum):
    LParen: 1
    RParen: 2


class Token:
    def __init__(self, type, value) -> None:
        self.Type = type
        self.value = value

    def __str__(self) -> str:
        return f'Token(Type: {self.Type.name}, Value: {self.value})'


class Lexer:
    def __init__(self, source) -> None:
        self.source = source
        self.tokens = []
        self.current = 0

    def tokenize(self):
        while self.current < len(self.source):
            char = self.source[self.current]

            if char == '(': 
                self.tokens.append(Token(TokenType.LParen, '('))
            elif char == ')':
                self.tokens.append(Token(TokenType.RParen, ')'))
            
            self.current += 1

    def print(self):
        print("\n===== TOKENS =====\n")
        for token in self.tokens:
            print(token)


def main():
    source = "(()()))asdsaasd)(())"

    lexer = Lexer(source)
    lexer.tokenize()
    lexer.print()


if __name__ == "__main__":
    main()
