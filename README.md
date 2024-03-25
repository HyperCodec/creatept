# My CreatePT Project
This project was made in a total of ~9 hours using Rust and the Bevy game engine.

My starting experience of Bevy (and gamedev in general) was very minimal (pretty much nothing aside from [a shader test](https://inflectrix.github.io/bevy-shader-test); the page may take a while to load, WASM/executables from Bevy are very large). In this project, I had to learn (self-taught off of textual documentation):
- Bevy's event system
- Rapier (physics engine)
- EGUI (popup/sidebar UI engine)
- Bevy's UI system
- Bevy project management/ECS (Plugins, Bundles, etc.)
- Blender (for modeling scenes)
- Hanabi (particle system)

And much more, all within the span of ~9 hours of development time (albeit drawn out to fit within 40min periods) to produce a playable/presentable game.

### Controls
- Mouse Movement - Aim
- WASD - Basic movement
- Shift - Sprint (also more airdrift when airborne)
- Ctrl - Crouch
- R - Spawn bomb (inherits player velocity, used to launch the player by positioning oneself near the bomb when it explodes)

### Asset Sources
You can find any asset sources I used [here](/assets/credits.txt)

### Copyright
This repository uses the `MIT` license.
