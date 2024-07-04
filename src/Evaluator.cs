namespace eval.src;

internal class Lexer(string source) 
{
    public string Source { get; set; } = source;
    public int Current { get; set; } = 0;
    public int Start { get; set; }
    public List<Token> Tokens { get; set; } = [];

    public List<Token> Tokenize() 
    {
        while (!IsEnd()) 
        {
            Start = Current;
            Tokens.Add(NextToken());

            Current++;
        }

        return Tokens.Where(e => e.Type != TokenType.BAD).ToList();
    }

    private Token NextToken()
    {
        char c = Source[Current];

        return c switch
        {
            '+' or '-' or '*' or '/' => new Token(TokenType.OP, c),
            >= '0' and <= '9' => NumberToken(),
            '(' or ')' => new Token(c == '(' ? TokenType.LEFT_PAREN : TokenType.RIGHT_PAREN, c),
            
            _ => new Token(TokenType.BAD, ""),
        };
    }

    private Token NumberToken()
    {
        bool decimalNumber = false;

        while (!IsEnd() && (char.IsDigit(Source[Current]) || (Source[Current] == '.' && !decimalNumber)))
        {
            if (Source[Current] == '.')
                decimalNumber = true;

            Current++;
        }

        Current--;
        return new Token(TokenType.NUMBER, Source[Start..(Current + 1)]);
    }

    private bool IsEnd()
        => Current >= Source.Length;
}