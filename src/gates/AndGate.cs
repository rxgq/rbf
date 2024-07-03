namespace cpu.src.gates;

public class AndGate : LogicGate
{
    public override void Update()
    {
        if (Inputs.Count < 2)
            throw new Exception("AND gate takes 2 or more inputs");

        bool outputSignal = true;

        foreach (var input in Inputs)
        {
            if (!input.Transistor.IsOn)
            {
                outputSignal = false;
                break;
            }
        }

        Output.Transistor.SwitchTo(outputSignal);
    }
}
