sealed class ItsMyBirthdayCard : MonopolyCard {

    public ItsMyBirthdayCard() {
        Value = 2;
    }

    public override string ToString(){
        return $"It's My Birthday: {Value}M";
    }
}