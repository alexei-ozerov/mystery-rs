# Mystery RS
A text-adventure, created with Rust and Rocket.

## Implementation
A multiplayer text adventure, where multiple players may act in turn to solve a mystery. Each game starts with generating a unique ID CODE for the play session. Data, messages, and activities are stored in a file on the server's host filesystem. When logging back in, entering a code that is already generated will result in the game picking back up at the last route that the players visited.
