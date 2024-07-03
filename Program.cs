using cpu.src;

namespace cpu;

class Program {
    static void Main() {
        var adder = new FullAdder();

        adder.A.Switch();
        adder.B.Switch();
        adder.C.Switch();

        adder.Run();
    }
}