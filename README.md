# RustyHUD

I've stolen my TUI window manager out of [The Computer Science House ImagineRIT 2020](https://github.com/ComputerScienceHouse/altctrl) project repo and made a crate out of it. My vision is a useful and powerful menu/HUD system for terminal-based games. Yes, you could probably get away with using [cursive](https://github.com/gyscos/cursive) or [tui-rs](https://github.com/fdehau/tui-rs) or something, but I wanted to use what I had already written to try to learn something! The main goal of this is to provide a dead-simple, lightweight, and unobtrusive way to create UI elements overtop of whatever else is in your terminal. Ideally, you should be able to toggle this like an overlay on interactive applications.

### Dependencies:

 - `ncurses`
 - `ncurses-dev`

### Garfanzo (Definitely busted right now but don't worry about it):
Garfanzo™ is a demo program.

To properly run Garfanzo, you need to pipe the output to another tty. You can do this from a desktop session like so:

Open two terminal emulators.
In the first terminal run the tty command. This command will return the tty device of the terminal emulator (eg. `/dev/pts/1`).
In the second terminal navigate to this repo and run,
 `cargo run -- -d -i=garfanzo > /dev/pts/1`

 To create a window, try this: `window new TITLE T MESSAGE plain 20 20 10 10`

