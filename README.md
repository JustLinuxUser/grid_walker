# My "cheat" of [spirodonfl](https://www.twitch.tv/spirodonfl) game.

> [!WARNING]  
> ## WARNING: This currently does not work, something to do with the Speedy2D library, or the most recent version of Hyprland or both

## How this works:
1. You copy the state of the game to the board (U, T, E keys)
2. You click on where you want to be, and the path finding algorithm will find a path
3. The app will copy the instructions to the clipboard along with some attack moves at the end, they should look like: `!r9r1d7a9a9a9a9a93549446981`, Note it adds a random number to the end to bypass the limitation of repeating messages on twitch
4. Paste the message in the chat.


This is not a real cheat, spirodonfl gave me permission to make and publish it, and he approves of my attempt of doing something cool. This has been made in 1 night, so the code quality is non existent
![](https://imgur.com/yZIuGEV.png)

## How to use it?
1. Clone this repo
2. Run `cargo run`
