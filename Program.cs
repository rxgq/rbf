using eval.src;

namespace eval;

internal class Program {
    static void Main() {
        var expression = "(1 + 2) * 4";

        var eval = new Evaluator(expression);
        var tokens = eval.Evaluate();

        foreach (var token in tokens) Console.Write(token.ToString());
    }
}