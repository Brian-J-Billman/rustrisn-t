# Rustrisn-t
Tetrisn-t rewritten in Rust, better.

`rustrisn-t.exe` is included on each tagged `master` branch commit so that Windows users don't have to compile the code.

Make sure your monitor's refresh rate is set to 60FPS; otherwise the game runs at a different speed than intended; this will eventually be fixed.

# Build
Download and install cargo, then
```
cargo build --release
```
Google any errors that occur, if they do.

# Gamepads
In the "Controls" menu, keyboard control schemes and setting players to use gamepads are both possible and mostly self explanitory.
It is possible to connect multiple keyboards to one PC and use both separately, but the inputs show up as the same, so the keyboard control schemes are not allowed to overlap, even across separate keyboards.
There is a check in place, which will give a notice when a key is attempted to be re-used.
Unfortunately, these settings are not saved, so they will have to be reconfigured each time the program is opened; eventually it will be implemented.

Setting a player to use a gamepad is easily done in the "Controls" menu by pressing 'G', as it states.
The game decides which specific gamepad controls which player by assigning a gamepad's id to the first player that doesn't yet have an assigned gamepad id as soon as an unassigned gamepad has a button/axis pressed during gameplay.
Because of weird coding stuff, this is done each game no matter what, but might eventually be extended to be the entire instance of the program.

## Custom Layouts and Obscure Compatibility
This program uses SDL2 gamepad configurations for controllers.  
A list of automatically-detected controllers can be found in the `resources` folder in `gamecontrollerdb.txt`, which is a copy of this file: https://github.com/gabomdq/SDL_GameControllerDB/blob/master/gamecontrollerdb.txt  
The `resources` folder with its contents must be in the same location as the binary (exe for Windows users) in order for gamepads to continue to be detected; create a shortcut to the binary if you want to open it from elsewhere.

In case custom layouts or compatibility for obscure gamepads are desired, here is a cross-platform SDL2 configurer: https://generalarcade.com/gamepadtool/. A quick guide on how to use it is as follows:  
1. Download, install, open  
2. Select desired gamepad in the dropdown on the top  
3. Select "Create A New Mapping"  
4. Create the desired mapping using input on your gamepad  
5. Select "Copy Mapping String" and paste the string into a newline of `gamecontrollerdb.txt`, deleting the line of the controller with the same Gamepad GUID (the first really long number) if it exists  
Then the gamepad should be recognized when the program is opened again. When creating a gamepad mapping, consider which buttons the program has set to do which action:
```
Axis::LeftAxisX- and Button::DPadLeft -> Left
Axis::LeftAxisX+ and Button::DPadRight -> Right
Axis::LeftAxisY- and Button::DPadDown -> Down
Button::West -> RotateCw
Button::South -> RotateCcw
Button::Start -> Start
```
where, in the graphic, `Button::Start` is the small button just to the right of the circle button in the middle, and the compass directions refer to the four buttons on the right.