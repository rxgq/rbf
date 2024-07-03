using cpu;

class NotGate : LogicGate {
    
    public NotGate() {
        Output.Transistor.Switch();
    }

    public override void Update() {
        if (Inputs.Count > 1)
            throw new Exception("NOT gate can only have one input.");

        
    }
}