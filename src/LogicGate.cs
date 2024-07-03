namespace cpu;

public abstract class LogicGate {
    public List<Wire> Inputs { get; set; } = new();
    public Wire Output { get; set; } = new();

    public bool State => Output.Transistor.IsOn;

    public abstract void Update();

    public override string ToString()
    {
        Console.Write("\n\n\n");

        string str = "";
        for (int i = 0; i < Inputs.Count; i++)
        {
            str += $"{i}| {Inputs[i].Transistor.IsOn}\n";
        }
        str += $"\nOut: {Output.Transistor.IsOn}";
        return str;
    }
}