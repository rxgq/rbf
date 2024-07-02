sealed class Processor {
    private ALU ALU { get; set; } = new();
    private ControlUnit ControlUnit { get; set; } = new();
    private Memory Memory { get; set; }

    private ProgramCounter Counter { get; set; } = new();
    private InstructionRegister Instruction { get; set; } = new();
    public GeneralRegister Accumulator { get; set; }

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
        
    }
}