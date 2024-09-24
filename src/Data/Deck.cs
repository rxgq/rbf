sealed class Deck {
    public List<MonopolyCard> Cards { get; set; } = new();

    private readonly Dictionary<MonopolyCard, int> _cardConfig = new() {
        {new DealBreakerCard(),   2},
        {new DebtCollectorCard(), 3},
        {new DoubleRentCard(),    2},
        {new ForcedDealCard(),    3},
        {new HotelCard(),         3},
        {new HouseCard(),         3},
        {new ItsMyBirthdayCard(), 3},
        {new JustSayNoCard(),     3},
        {new PassGoCard(),       10},
        {new SlyDealCard(),       3},

        

        {new RentCard(RentType.AllColors, []), 3},
        {new RentCard(RentType.TwoColors, [PropertyColour.Green, PropertyColour.Blue]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Brown, PropertyColour.LightBlue]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Purple, PropertyColour.Orange]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Black, PropertyColour.LightGreen]), 2},
        {new RentCard(RentType.TwoColors, [PropertyColour.Red, PropertyColour.Yellow]), 2},

        {new MoneyCard(1), 10},
        {new MoneyCard(2),  5},
        {new MoneyCard(3),  3},
        {new MoneyCard(4),  3},
        {new MoneyCard(5),  2},
        {new MoneyCard(10), 1},

        {new PropertyCard(2, PropertyColour.Brown, "Baltic Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Brown, "Mediterranean Avenue"), 1},

        {new PropertyCard(2, PropertyColour.LightGreen, "Water Works"), 1},
        {new PropertyCard(2, PropertyColour.LightGreen, "Electric Company"), 1},

        {new PropertyCard(2, PropertyColour.LightBlue, "Connecticut Avenue"), 1},
        {new PropertyCard(2, PropertyColour.LightBlue, "Oriental Avenue"), 1},
        {new PropertyCard(2, PropertyColour.LightBlue, "Vermont Avenue"), 1},

        {new PropertyCard(2, PropertyColour.Red, "Kentucky Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Red, "Indiana Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Red, "Illinois Avenuew"), 1},

        {new PropertyCard(2, PropertyColour.Yellow, "Ventnor Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Yellow, "Marvin Gardens"), 1},
        {new PropertyCard(2, PropertyColour.Yellow, "Atlantic Avenue"), 1},

        {new PropertyCard(2, PropertyColour.Green, "North Carolina Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Green, "Pacific Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Green, "Pennsylvania Avenue"), 1},

        {new PropertyCard(2, PropertyColour.Blue, "Boardwalk"), 1},
        {new PropertyCard(2, PropertyColour.Blue, "Park Place"), 1},

        {new PropertyCard(2, PropertyColour.Orange, "New York Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Orange, "St. James Place"), 1},
        {new PropertyCard(2, PropertyColour.Orange, "Tennessee Avenue"), 1},

        {new PropertyCard(2, PropertyColour.Purple, "St. Charles Place"), 1},
        {new PropertyCard(2, PropertyColour.Purple, "Virginia Avenue"), 1},
        {new PropertyCard(2, PropertyColour.Purple, "States Avenue"), 1},

        {new PropertyCard(2, PropertyColour.Black, "Short Line"), 1},
        {new PropertyCard(2, PropertyColour.Black, "B. & O. Railroad"), 1},
        {new PropertyCard(2, PropertyColour.Black, "Reading Railroad"), 1},
        {new PropertyCard(2, PropertyColour.Black, "Pennsylvania Railroad"), 1},
    };
    public Deck() {
        foreach (var card in _cardConfig) {
            
            for (int i = 0; i < card.Value; i++) 
                Cards.Add(card.Key);
        }
    }
}