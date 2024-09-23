class Program {
    static void Main() {
        var config = new MonopolyConfig() {

        };

        var controller = new MonopolyController(config);

        var monopoly = new Monopoly(config, controller);
        monopoly.Start();
    }
}