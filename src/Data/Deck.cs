sealed class Deck {
    public List<MonopolyCard> Cards { get; set; } = new();

    private readonly Dictionary<MonopolyCard, int> _cardConfig = new() {
        {new TaxCard(2), 4},
        {new TaxCard(5), 4},

        {new RentCard(RentType.AllColors, []), 4},
        {new RentCard(RentType.TwoColors, [PropertyColour.Red, PropertyColour.Yellow]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.LightGreen, PropertyColour.LightBlue]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Blue, PropertyColour.Green]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Brown, PropertyColour.Orange]), 2},

        {new DoubleRentCard(), 4},
        {new SwapCard(), 4},
        {new StealCard(), 4},
        {new JustSayNoCard(), 3},
        {new DealBreakerCard(), 2},

        {new MoneyCard(1), 10},
        {new MoneyCard(2),  7},
        {new MoneyCard(3),  5},
        {new MoneyCard(4),  5},
        {new MoneyCard(5),  4},
        {new MoneyCard(10), 2},

        {new PropertyCard(2, PropertyColour.Brown, "Brown Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Brown, "Brown Street 2"), 1},

        {new PropertyCard(2, PropertyColour.LightGreen, "Light Green Street 1"), 1},
        {new PropertyCard(2, PropertyColour.LightGreen, "Light Green Street 2"), 1},

        {new PropertyCard(2, PropertyColour.LightBlue, "Light Blue Street 1"), 1},
        {new PropertyCard(2, PropertyColour.LightBlue, "Light Blue Street 2"), 1},

        {new PropertyCard(2, PropertyColour.Red, "Red Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Red, "Red Street 2"), 1},
        {new PropertyCard(2, PropertyColour.Red, "Red Street 3"), 1},

        {new PropertyCard(2, PropertyColour.Yellow, "Yellow Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Yellow, "Yellow Street 2"), 1},
        {new PropertyCard(2, PropertyColour.Yellow, "Yellow Street 3"), 1},

        {new PropertyCard(2, PropertyColour.Green, "Green Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Green, "Green Street 2"), 1},
        {new PropertyCard(2, PropertyColour.Green, "Green Street 3"), 1},

        {new PropertyCard(2, PropertyColour.Blue, "Blue Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Blue, "Blue Street 2"), 1},

        {new PropertyCard(2, PropertyColour.Orange, "Orange Street 1"), 1},
        {new PropertyCard(2, PropertyColour.Orange, "Orange Street 2"), 1},
        {new PropertyCard(2, PropertyColour.Orange, "Orange Street 3"), 1},

        {new PropertyCard(2, PropertyColour.Purple, "Purple Street 3"), 1},
        {new PropertyCard(2, PropertyColour.Purple, "Purple Street 3"), 1},
        {new PropertyCard(2, PropertyColour.Purple, "Purple Street 3"), 1},
    };
    public Deck() {
        foreach (var card in _cardConfig) {
            
            for (int i = 0; i < card.Value; i++) 
                Cards.Add(card.Key);
        }
    }
}