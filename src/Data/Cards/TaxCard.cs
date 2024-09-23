sealed class TaxCard : MonopolyCard {
    public int TaxToPay { get; set; }

    public TaxCard(int amount) {
        TaxToPay = amount;
    }

    public override string ToString(){
        return $"Tax a Player: {TaxToPay}M";
    }
}