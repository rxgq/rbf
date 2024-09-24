sealed class DebtCollectorCard : MonopolyCard {

    public DebtCollectorCard() {
        Value = 5;
    }

    public override string ToString(){
        return $"Debt Collector: {Value}M";
    }
}