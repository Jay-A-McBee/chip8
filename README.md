# Chipwich

#### A rustaceous chip8 emulator
###### Do you long for the TI graphing calculator games of your youth?

### Requirements

#### SDL Development Library

##### macOS
```
brew install sdl2
```
##### Linux (ubuntu)
```
sudo apt-get install libsdl2-dev
```
See the [SDL2.0 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) section of the rust-sdl2 library for more information or to troubleshoot install issues.

### Start the CLI

```
cargo run
```

### CLI Options
###### Select a Game
Play one of the locally installed games or checkout a visual demo/test in the games directory.

###### Upload a Game
Enter a file path to upload a game locally from disk.

###### Download a Game
Enter an http/s url and download a game from a remote server. For Example,
```
https://johnearnest.github.io/chip8Archive/roms/snake.ch8
```

### How do I play?

The original chip8 keyboard was a 16 key hexadecimal key pad. This has been mapped to the following modern keyboard layout.

```
 chip8      keyboard
-------     --------
1 2 3 C     1 2 3 4
4 5 6 D     Q W E R
7 8 9 E     A S D F
A 0 B F     Z X C V
```

Game play isn't the easiest to figure out. 2, S, Q, E sometimes is up, down, left, right. W starts games occasionally. I recommend downloading and starting with a classic game - [snake](https://johnearnest.github.io/chip8Archive/roms/snake.ch8). 

### Why is the code so verbose?
This was mostly an exercise in Rust completed by a person learning Rust. I wanted to leverage lang features (traits, trait objects, etc.) in a somewhat less contrived manner than the examples/exercises in "the book". If you notice something particularly terrible about the code, please let me know.

