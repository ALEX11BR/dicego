# dicego
Roll dice around the infinitely many procedurally generated levels so that they have the same value on top, then go to the stairs that will lead you into the next level. You have 60 seconds to finish one level. Levels get harder the deeper you go in.

## Controls:
- Arrow keys: move the player
- Enter: finish the current level and enter the next one, if you are standing over the stairs to the next level.

## Build
Just like any rust program, `cargo build --release`. Note that the `img` directory (you can safely remove its svg's) must be in the same directory as the executable.