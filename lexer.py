from tokens import Token, TokenType, KEYWORDS

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
            elif str.isnumeric(char):
                self.tokens.append(self.on_numeric())
            elif char == ' ': pass
            else:
                self.tokens.append(self.on_identifier())

            self.current += 1

    def on_identifier(self) -> Token:
        start = self.current

        while str.isalpha(self.peek()) or self.peek() in '+-*/=<>&|!':
            self.current += 1

        identifier = self.source[start:self.current + 1]
        token_type = KEYWORDS.get(identifier, TokenType.Identifier)

        return Token(token_type, identifier)

    def on_numeric(self):
        start = self.current

        while str.isnumeric(self.peek()):
            self.current += 1

        return Token(TokenType.Number, self.source[start:self.current + 1])

    def peek(self) -> chr:
        if self.current < len(self.source):
            return self.source[self.current + 1]
        
        return '\0'

    def print(self):
        print("\n===== TOKENS =====\n")
        for token in self.tokens:
            print(token)