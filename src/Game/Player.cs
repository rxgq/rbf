sealed class Player {
    public string Name { get; set; }
    public Deck Deck { get; set; } = new();

    public List<MonopolyCard> PlayedCards = [];

    public Player(string name) {
        Name = name;
    }
}