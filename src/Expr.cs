namespace eval.src;

internal abstract class Expr {}

internal class BinaryExpr(Expr left, Expr right, Token op) : Expr {
    public Expr Left { get; } = left;
    public Expr Right { get; } = right;
    public Token Op { get; } = op;

    public override string ToString() {
        return $"({Op.Value} {Left} {Right})";
    }
}

internal class GroupExpr(Expr expression) : Expr {
    public Expr Expression { get; } = expression;

    public override string ToString() {
        return $"(Group {Expression})";
    }
}

internal class LiteralExpr(double value) : Expr {
    public double Value { get; } = value;

    public override string ToString() {
        return $"{Value}";
    }
}
