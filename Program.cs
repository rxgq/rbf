using cpu.src;

namespace cpu.src.components;

class Program {
    static void Main() {
        var num1 = "01100111";
        var num2 = "11100011";
        var fullAdder8 = new FullAdder8(num1, num2);
        fullAdder8.Run();

    }
}