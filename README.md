# OfficeQuest
Deckbuilding Game built in rust.

Take on the role of a nameless office worker, doing your best to make a name for yourself and not be another cog in the machine. Using your smarts you will manuever through a variety of social combat situations and random events. Enemies include: HR Representative, Incompetent Supervisor, Rival Department Head, etc. Random events may occur such as: Pizza Party, Overtime Request, Withheld Wages, etc.

## Additional Information
Author: Tom Jacobson
Project Name: Office Quest

As stated before this is a deckbuilder game created in rust. The current version is a MVP and not the fully functional game. Currently the game allows for terminal based play only, no GUI is implemented.
To build and run the project simply clone it and perform cargo run. You should then be able to play in the terminal (game window must be in the foreground to register key inputs).
Testing was mainly done with me as the sole play tester. A small suite of tests were implemented but these should also be expanded as the project is continued to be worked on.

The below code represents the majority of the application as it runs. Using ggez crate state is updated and changed occordingly. See implementation for more details.
```
   let c = Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OfficeQuest", "Tom Jacobson")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
```

## How to play (in current state)
'cargo run' will compile the game and open a blank game window. Ensure that this window remains in the foreground as this will allow the game to detect key inputs. All information will be displayed in the terminal so I recommend expanding it to cover a majority of your screen. I did my best to create visual clarity using color and dividing lines but I'm sure it will be somewhat confusing at first glance. All desired key inputs will be prompted for on screen. For example: 
```
Turn: PlayerTurn
------------------
Player Health: 10/10
Current Energy: 3/3
Enemy Health: 4
------------------
Tom's Hand:
********************************************************************** 
Press 1 to play card.
Card Name: Water Cooler
Mana Cost: 3
Textbox: Heal 1hp at the end of each of your turns.
Flavor Text: "The town square of gossipmongers and time thieves alike."
-------------------------------------------------------------------    
Press 2 to play card.
Card Name: Pizza Party
Mana Cost: 2
Textbox: Heal 2hp.
Flavor Text: "In lieue of quarterly bonuses corporate is giving us a Pizza Party!"
-------------------------------------------------------------------
Press 3 to play card.
Card Name: Friday Meeting
Mana Cost: 1
Textbox: Deal 4 damage to any target, take 2 damage.
Flavor Text: "Who scheduled the meeting for Friday at 5pm?"
-------------------------------------------------------------------
**********************************************************************
```

This should be the initial screen shown as the game starts you off in a combat. You will see all relevant combat information at the top (health, energy, and enemy name/health). Below you will see your opening hand of cards and their effects. Additionally you will see a number over the card telling you which key will play it. Press Spacebar to end your turn. Future combats and events will display information similarly. If at any point you get stuck and cannot continue the game loop you may press ctrl+c in the terminal to stop the program. 

## What worked/Didn't work

The game loop itself works great. Updating things via states is a very effective way to manage the game, though there was a learning curve on how to actually use those features. It always astounds me how much work goes into making a functional game, even one like this where "functional" is a term used very loosely in it's current state. 

Desired Future Improvements: 
* More events, enemies, and cards.
* Less simplistic game loop (currently combat->event->combat->event ad nauseam) 
* Additional card types and combat mechanics (currently just damage and healing)
* GUI

## License

This project is licensed under the [MIT License](LICENSE). 

## AI Assistance Disclosure

This project includes code that was initially generated with AI assistance (Google Gemini & ChatGPT).  
Significant modifications, debugging, and improvements were made to ensure accuracy, efficiency, and integration  
within the project's scope.
