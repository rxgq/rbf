namespace cpu.src;

internal class XorGate : LogicGate
{

    public override void Update()
    {
        if (Inputs.Count < 2)
            throw new Exception("XOR gate takes 2 or more inputs");

        int trueCount = 0;
        foreach (var input in Inputs)
        {
            if (input.Transistor.IsOn)
                trueCount++;
        }

        bool outputSignal = (trueCount % 2 != 0);
        Output.Transistor.SwitchTo(outputSignal);
    }
}
