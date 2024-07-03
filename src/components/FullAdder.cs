using cpu.src.gates;

namespace cpu.src.components;

internal class FullAdder
{
    public Wire A { get; set; } = new();
    public Wire B { get; set; } = new();
    public Wire C { get; set; } = new();

    public FullAdder(bool carryIn) 
    {
        if (carryIn)
            C.Transistor.SwitchTo(true);
    }

    public bool Run()
    {
        var xor1 = new XorGate();
        xor1.Inputs.AddRange(new List<Wire>() { A, B });

        var xor2 = new XorGate();
        xor2.Inputs.AddRange(new List<Wire>() { xor1.Output, C });

        var and1 = new AndGate();
        and1.Inputs.AddRange(new List<Wire>() { C, xor1.Output });

        var and2 = new AndGate();
        and2.Inputs.AddRange(new List<Wire>() { B, A });

        var or1 = new OrGate();
        or1.Inputs.AddRange(new List<Wire>() { and1.Output, and2.Output });

        xor1.Update();
        xor2.Update();
        and1.Update();
        and2.Update();
        or1.Update();
        
        //Console.Write(xor1.ToString() + " XOR1");
        //Console.Write(and1.ToString() + " AND1");
        //Console.Write(and2.ToString() + " AND2");

        //Console.Write(or1.ToString() + " OR1");  // carry out
        //Console.Write(xor2.ToString() + " XOR2"); // output

        Console.WriteLine(xor2.State ? "1" : "0");

        return or1.State;
    }
}
