namespace cpu;

public class Wire
{
    public Transistor Transistor { get; private set; } = new Transistor();
    public List<LogicGate> Connections { get; private set; } = new();

    public void Switch() {
        Transistor.Switch();
    }

    public void Connect(LogicGate gate) {
        Connections.Add(gate);
    }
}
