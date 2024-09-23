enum RentType {
    TwoColors,
    AllColors
}

sealed class RentCard : MonopolyCard {
    public RentType Type { get; set; }
    public List<PropertyColour> Colours { get; set; }

    public RentCard(RentType type, List<PropertyColour> colours) {
        Type = type;
        Colours = colours;
    }

    public override string ToString() {
        var colorsStr = "";
        foreach (var colour in Colours) {
            colorsStr += $"{colour}, ";
        }

        var typeStr = Type switch {
            RentType.TwoColors => $"Charge rent: {colorsStr[0..(colorsStr.Length - 2)]}",
            RentType.AllColors => $"Charge rent: Any Colour",
            _                  => "Unkown.",
        };
        
        return $"{typeStr}";
    }
}