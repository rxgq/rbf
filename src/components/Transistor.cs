namespace cpu.src.components;

public class Transistor
{
    public bool IsOn { get; private set; }

    public void Switch()
        => IsOn = !IsOn;

    public void SwitchTo(bool signal)
        => IsOn = signal;
}
