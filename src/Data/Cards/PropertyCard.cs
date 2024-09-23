sealed class PropertyCard : MonopolyCard {
    public PropertyColour Colour { get; set; }
    public string Name { get; set; }

    public PropertyCard(int value, PropertyColour colour, string name) {
        Colour = colour;
        Value = value;
        Name = name;
    }

    public override string ToString() {
        //return $"{Name} - {Colour}: {Value}";
        return $"{Name}: {Value}M";
    }
}