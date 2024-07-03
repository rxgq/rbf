namespace cpu.src.components;

internal class FullAdder8
{
    public bool A1 { get; set; }
    public bool A2 { get; set; }
    public bool A3 { get; set; }
    public bool A4 { get; set; }
    public bool A5 { get; set; }
    public bool A6 { get; set; }
    public bool A7 { get; set; }
    public bool A8 { get; set; }
    public bool B1 { get; set; }
    public bool B2 { get; set; }
    public bool B3 { get; set; }
    public bool B4 { get; set; }
    public bool B5 { get; set; }
    public bool B6 { get; set; }
    public bool B7 { get; set; }
    public bool B8 { get; set; }


    public FullAdder8(string num1, string num2)
    {
        if (num1.Length != 8 || num2.Length != 8)
            throw new ArgumentException("Input strings must be 8 characters long.");

        A1 = num1[7] == '1';
        A2 = num1[6] == '1';
        A3 = num1[5] == '1';
        A4 = num1[4] == '1';
        A5 = num1[3] == '1';
        A6 = num1[2] == '1';
        A7 = num1[1] == '1';
        A8 = num1[0] == '1';

        B1 ^= num2[7] == '1';
        B2 ^= num2[6] == '1';
        B3 ^= num2[5] == '1';
        B4 ^= num2[4] == '1';
        B5 ^= num2[3] == '1';
        B6 ^= num2[2] == '1';
        B7 ^= num2[1] == '1';
        B8 ^= num2[0] == '1';
    }

    public void Run() 
    {
        var adder1 = new FullAdder(false);
        adder1.A.Transistor.SwitchTo(A1);
        adder1.B.Transistor.SwitchTo(B1);
        var carryOut1 = adder1.Run();

        var adder2 = new FullAdder(carryOut1);
        adder2.A.Transistor.SwitchTo(A2);
        adder2.B.Transistor.SwitchTo(B2);
        var carryOut2 = adder2.Run();

        var adder3 = new FullAdder(carryOut2);
        adder3.A.Transistor.SwitchTo(A3);
        adder3.B.Transistor.SwitchTo(B3);
        var carryOut3 = adder3.Run();

        var adder4 = new FullAdder(carryOut3);
        adder4.A.Transistor.SwitchTo(A4);
        adder4.B.Transistor.SwitchTo(B4);
        var carryOut4 = adder4.Run();

        var adder5 = new FullAdder(carryOut4);
        adder5.A.Transistor.SwitchTo(A5);
        adder5.B.Transistor.SwitchTo(B5);
        var carryOut5 = adder5.Run();

        var adder6 = new FullAdder(carryOut5);
        adder6.A.Transistor.SwitchTo(A6);
        adder6.B.Transistor.SwitchTo(B6);
        var carryOut6 = adder6.Run();

        var adder7 = new FullAdder(carryOut6);
        adder7.A.Transistor.SwitchTo(A7);
        adder7.B.Transistor.SwitchTo(A7);
        var carryOut7 = adder7.Run();

        var adder8 = new FullAdder(carryOut7);
        adder8.A.Transistor.SwitchTo(A8);
        adder8.B.Transistor.SwitchTo(B8);
        var carryOut8 = adder8.Run();

        Console.Write(carryOut8 ? "1": "0");
    }
}
