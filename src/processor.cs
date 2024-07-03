sealed class Processor {
    private ALU ALU { get; set; } = new();
    private ControlUnit ControlUnit { get; set; } = new();
    private Memory Memory { get; set; } = new(sizeBytes: 1024);

    private ProgramCounter ProgramCounter { get; set; } = new();
    private InstructionRegister InstructionRegister { get; set; } = new();
    private GeneralRegister Accumulator { get; set; }

    //private StackPointer Stack { get; set; } = new();
    //private FlagsRegister Flags { get; set; } = new();
    //private List<GeneralRegister> GeneralRegisters = [];


    public Processor() {

    }

    public void Run() {
        while (true) {
            Cycle();
        }
    }

    private void Cycle() {
        int? opCode = Memory.Fetch(ProgramCounter.Value);
        InstructionRegister.OpCode = ControlUnit.Decode(opCode ?? 1);

        Console.Write("Instruction is null, continuing...\n");
        Thread.Sleep(1000);
    }
}