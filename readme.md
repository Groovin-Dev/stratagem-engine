# Stratagem Engine

Stratagem Engine is a small macro executor allowing for quick execution of Helldivers 2 stratagems.
It is meant to be used with a Stream Deck and Advanced Launcher

## Features
- Quick execution of stratagems
- Customizable keybinds
- Customizable stratagem list

## Installation
1. Download the latest release from the [releases page]()
2. Extract the zip file
3. Run `stratagem_engine.exe --generate` and follow the instructions (This is only needed once to generate your keybinds file)
4. Install [Advanced Launcher](https://marketplace.elgato.com/product/advanced-launcher-d9a289e4-9f61-4613-9f86-0069f5897125)
5. Create a new action in Advanced Launcher for any Stratagem you want to use
   1. Set the arguments to `--stratagem "<stratagem_name>"`
   2. Set `Run As Administrator` to `True`
   3. Set `Run in Background` to `True`
6. Now, just press your Stream Deck button to call the stratagem

## Logs

Whenever a stratagem is called, a log is created in the `logs` folder. This log contains:
- The full args
- The full config
- The full list of loaded Stratagems
- The stratagem that was called
- The init key
- The action that was called
- The delay between actions

## Stratagem List

The full list of Stratagems can be found in the `stratagems.toml` file. You can add or remove stratagems from this file as you see fit.
You must use the exact name of the stratagem as it appears in the file when creating an action in Advanced Launcher. For example,
to call `Expendable Anti-Tank` you would use `stratagem_engine.exe --stratagem "Expendable Anti-Tank"`