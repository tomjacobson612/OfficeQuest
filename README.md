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
