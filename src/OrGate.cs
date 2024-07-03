using cpu;

class OrGate : LogicGate {

    public override void Update() {
        if (Inputs.Count < 2)
            throw new Exception("OR gate takes 2 or more inputs");

        foreach (var input in Inputs) 
        {
            if (input.Transistor.IsOn) 
            {
                Output.Transistor.SwitchTo(true);
                return;
            }
        }
    }
}