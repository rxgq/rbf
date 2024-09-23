
// this is not an actual monopoly card, it serves as an easy method to allow a user to
// select a "card" that skips their current turn
sealed class SkipTurn : MonopolyCard {
    public override string ToString() {
        return "Skip this turn";
    }
}