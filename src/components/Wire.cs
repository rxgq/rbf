using cpu.src.gates;

namespace cpu.src.components;

public class Wire
{
    public Transistor Transistor { get; private set; } = new Transistor();

    public void Switch() => Transistor.Switch();
}
