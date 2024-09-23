sealed class MonopolyController {
    private MonopolyConfig _config;
    public Deck PlayingDeck;
    public int PlayerCount = 2;
    public int PlayerTurn = -1;
    public List<Player> Players { get; set; } = [];
    public Player CurrentPlayer => Players[PlayerTurn - 1];

    public MonopolyController(MonopolyConfig config) {
        _config = config;
        PlayingDeck = new();
    }

    public void DrawTwo() {
        Console.WriteLine($"You drew 2 cards from the pile.");

        var cardOne = DrawRandomCard();
        CurrentPlayer.Deck.Cards.Add(cardOne);

        var cardTwo = DrawRandomCard();
        CurrentPlayer.Deck.Cards.Add(cardTwo);

        Console.Write($"\n\nDrew: {cardOne}\n");
        Console.Write($"Drew: {cardTwo}");

        Console.ReadKey();
    }

    public Deck CreatePlayerDeck() {
        var deck = new Deck();
        deck.Cards.Clear();

        for (int i = 0; i < 5; i++) {
            var card = DrawRandomCard();
            deck.Cards.Add(card);
        }

        return deck;
    }

    public MonopolyCard DrawRandomCard() {
        var rnd = new Random();

        var cardIdx = rnd.Next(0, PlayingDeck.Cards.Count);
        var card = PlayingDeck.Cards[cardIdx];
        
        PlayingDeck.Cards.Remove(card);

        return card;
    }

    public void Discard(MonopolyCard card) {
        CurrentPlayer.Deck.Cards.Remove(card);
    }

    public void PlayCard(MonopolyCard card) {
        CurrentPlayer.Deck.Cards.Remove(card);
        CurrentPlayer.PlayedCards.Add(card);
    }
    
    public void NextTurn() {
        if (PlayerTurn == PlayerCount) PlayerTurn = 1;
        else PlayerTurn++; 
    }
}