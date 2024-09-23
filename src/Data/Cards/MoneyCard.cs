sealed class MoneyCard : MonopolyCard {
    public MoneyCard(int value) {
        Value = value;
    }

    public override string ToString() {
        return $"Money {Value}M";
    }
}