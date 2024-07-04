namespace eval.src;

internal class Token(TokenType type, object value) {
    public TokenType Type { get; set; } = type;
    public object Value { get; set; } = value;

    public override string ToString()
        => $"Token: ({Type,-14} | {Value,-4})\n";
}

public enum TokenType {
    OP,
    NUMBER,
    LEFT_PAREN,
    RIGHT_PAREN,
    BAD
}