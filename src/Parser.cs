namespace eval.src;

internal class Parser(List<Token> tokens)
{
    private List<Token> Tokens { get; init; } = tokens;
    private int _current = 0;

    public List<Expr> Parse()
    {
        var expressions = new List<Expr>();
        
        while (!IsAtEnd()) 
        {
            expressions.Add(Expression());
            _current++;
        }

        return expressions;
    }

    private Expr Expression()
    {
        return Addition();
    }

    private Expr Addition() 
    {
        var expr = Multiplication();


    }

    private Expr Multiplication() 
    {
        var expr = Primary();


    }

    private LiteralExpr Primary() 
    {
        var token = Tokens[_current];

        if (Check(TokenType.NUMBER))
            return new LiteralExpr(Convert.ToDouble(token.Value));

        throw new Exception("Unexpected token."); 
    }

    private bool Check(TokenType type) {
        if (IsAtEnd()) return false;
        return Tokens[_current].Type == type;
    }

    private bool IsAtEnd()
        => _current >= Tokens.Count;
}
