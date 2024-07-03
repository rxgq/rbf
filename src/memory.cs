sealed class Memory {
    public byte?[] RAM { get; set; }

    public Memory(int sizeBytes) {
        RAM = new byte?[sizeBytes];
        InitializeMemory(sizeBytes);
    }

    public void InitializeMemory(int size) {
        for (int i = 0; i < size; i++) {
            RAM[i] = null;
        }
    }

    public byte? Fetch(int counter)
        => RAM[counter];
}