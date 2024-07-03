public enum OpCode {
    LOAD = 0x00,
    RESET = 0x01
}

sealed class ControlUnit {
    public Dictionary<int, OpCode> Instructions = new() { 
        { 0x00, OpCode.LOAD },
        { 0x01, OpCode.RESET }
    };

    public OpCode Decode(int opCode) 
        => Instructions[opCode];
}