using Microsoft.VisualBasic;

static class Display {
    public static T Menu<T>(string header, List<T> options) {
        int idx = 0;

        while (true) {
            Console.Clear();

            Console.WriteLine($"{header}\n");
            for (int i = 0; i < options.Count; i++) {
                Console.WriteLine(idx == i ? $"> {options[i]}" : options[i]);
            }

            switch (Console.ReadKey().Key) {
                case ConsoleKey.UpArrow:
                    if (idx != 0) idx--; 
                    else idx = options.Count - 1; break;
                case ConsoleKey.DownArrow:
                    if (idx != options.Count - 1) idx++;
                    else idx = 0;  break;

                case ConsoleKey.Enter:
                    return options[idx];
            }
        }
    }

    public static void ReadKey(string message) {
        Console.WriteLine(message);
        Console.ReadKey();
    }
}