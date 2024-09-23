sealed class Monopoly {
    private readonly MonopolyConfig _config;
    private readonly MonopolyController _game;

    public Monopoly(MonopolyConfig config, MonopolyController controller) {
        _config = config;
        _game = controller;
    }

    private void Init() {
        for (int i = 0; i < _game.PlayerCount; i++) {
            _game.Players[i].Deck = _game.CreatePlayerDeck();
        }

        var rnd = new Random();

        var playerIdx = rnd.Next(1, _game.PlayerCount - 1);
        _game.PlayerTurn = playerIdx;
    }

    public void Start() {
        while (true) {
            Console.ForegroundColor = ConsoleColor.White;
            var option = Display.Menu("Welcome to the Monopoly Card Game!", ["Play", "Exit"]);

            if (option == "Play") break;
            else return;
        }

        var playerCount = 0;
        while (playerCount <= 8) {
            Console.Clear();

            Console.Write("Enter player name: ");
            var name = Console.ReadLine();

            if (string.IsNullOrWhiteSpace(name)) {
                Display.ReadKey("Please ensure that the players name is not empty\n");
                continue;
            }

            _game.Players.Add(new(name));
            playerCount++;

            Display.ReadKey($"Added player: '{name}'");

            if (playerCount == 8) {
                Console.WriteLine("\nThe maximum of 8 players have been added.\nThe game will now start.\n\nPress enter");
                Console.ReadKey();
                break;
            }

            var addAnotherPlayer = Display.Menu("Add another player?", ["Yes", "No"]);
            if (addAnotherPlayer == "No") break;
        }

        Init();
        
        Console.Clear();
        Display.ReadKey($"{_game.CurrentPlayer.Name} goes first!\n\n\tPress enter to begin the game.");

        while (true) {
            PlayTurn();
        }
    }

    public void PlayTurn() {
        Console.Clear();

        Display.ReadKey($"It's your turn, {_game.CurrentPlayer.Name}.\nPress enter to draw two cards.\n");
        _game.DrawTwo();

        int cardsPlayed = 0;
        while (true) {
            var cards = _game.CurrentPlayer.Deck.Cards.Select(c => c).ToList();
            cards.Add(new SkipTurn());
            
            var chosenCard = Display.Menu(
                $"\nHere is your deck, {_game.CurrentPlayer.Name}:\nYou have {3 - cardsPlayed} turns left", 
                cards
            );

            if (chosenCard is MoneyCard) {
                _game.PlayCard(chosenCard);
                Display.ReadKey($"\n\t- Used: {chosenCard}, added to funds.");
            } else if (chosenCard is SkipTurn) {
                Display.ReadKey("\n\t- Skipping this turn..\n\t- Press enter");
                break;
            }

            else continue;

            cardsPlayed++;
            if (cardsPlayed == 3) break;
        }

        while (_game.CurrentPlayer.Deck.Cards.Count > 7) {
            var option = Display.Menu("Choose a card to discard", _game.CurrentPlayer.Deck.Cards);
            _game.Discard(option);

            Display.ReadKey($"\nDiscarded: {option}");
        }

        _game.NextTurn();
    }
}