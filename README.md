# BfBB Clash

BfBB Clash is a custom gamemode for SpongeBob SquarePants: Battle for Bikini Bottom where up to six players compete for the same set spatulas within the game world. Once one player obtains a spatula, that spatula is removed from the game world for every other player.

## How to Play

- Have dolphin open with the game running.
- Open the client and join a lobby.
- With all players on the Main Menu the game will start automatically when the host starts a new game.

## Building From Source
* Install [Rust](https://www.rust-lang.org/tools/install)

The code is separated into three packages: the shared library `clash`, `client`, and `server`.
With cargo from the project root:
- `$ cargo build` Build all packages
- `$ cargo build -p <package>` Build a single package
