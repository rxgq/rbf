namespace eval.src;

internal class Parser(List<Token> tokens)
{
    private readonly List<Token> Tokens = tokens;
    private int _current = 0;

    public List<Expr> Parse()
    {
        var expressions = new List<Expr>();

        while (!IsAtEnd())
            expressions.Add(Expression());

        return expressions;
    }

    private Expr Expression()
    {
        return Addition();
    }

    private Expr Addition()
    {
        Expr expr = Multiplication();

        while (Match(TokenType.OP, "+", "-"))
        {
            Token op = PreviousToken();
            Expr right = Multiplication();
            expr = new BinaryExpr(expr, right, op);
        }

        return expr;
    }

    private Expr Multiplication()
    {
        var expr = Primary();

        while (Match(TokenType.OP, "*", "/"))
        {
            Token op = PreviousToken();
            Expr right = Primary();
            expr = new BinaryExpr(expr, right, op);
        }

        return expr;
    }

    private Expr Primary()
    {
        var token = CurrentToken();

        if (Match(TokenType.NUMBER))
            return new LiteralExpr(Convert.ToDouble(token.Value));

        throw new Exception("Unexpected token.");
    }

    private Token CurrentToken()
        => Tokens[_current];

    private bool Check(TokenType type)
    {
        if (IsAtEnd()) return false;
        return Tokens[_current].Type == type;
    }

    private bool Match(TokenType type)
    {
        if (Check(type))
        {
            _current++;
            return true;
        }

        return false;
    }

    private bool Match(TokenType type, params string[] values)
    {
        foreach (var value in values)
        {
            if (Check(type) && Tokens[_current].Value.ToString() == value)
            {
                _current++;
                return true;
            }
        }

        return false;
    }

    private Token PreviousToken()
        => Tokens[_current - 1];

    private bool IsAtEnd()
        => _current >= Tokens.Count;
}