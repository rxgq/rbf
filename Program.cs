using eval.src;

namespace eval;

internal class Program {
    static void Main() {
        var expression = "1 + 2 * 4";

        var lexer = new Lexer(expression);
        var tokens = lexer.Tokenize();

        var parser = new Parser(tokens);
        var exprs = parser.Parse();

        foreach (var expr in exprs)
            Console.Write(expr.ToString());

        // foreach (var token in tokens) 
        //     Console.Write(token.ToString());
    }
}