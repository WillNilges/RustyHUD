# RustyHUD

So I'm stealing my TUI window manager code stuff out of [The Computer Science House ImagineRIT 2020](https://github.com/ComputerScienceHouse/altctrl) project repo and I guess I'm gonna make a crate out of it. My vision is a useful and powerful menu/HUD system for terminal-based games. Yes, you could probably get away with using [cursive](https://github.com/gyscos/cursive) or [tui-rs](https://github.com/fdehau/tui-rs) or something, but fuck it! I'm gonna do it my way! I guess the main goal of this is to provide unobtrusive UI elements overtop of whatever else is in your terminal. Ideally, you should be able to toggle this like an overlay on interactive applications. I don't really know! Also, the other libraries I mentioned use a bunch of stuff I've never heard of (not that that's a bad thing), whereas this just needs ncurses.

### Dependencies:

 - `ncurses`
 - `ncurses-dev`

### Garfanzo (Definitely busted right now but don't worry about it):
Garfanzo™ is our debug program.

To properly run Garfanzo, you need to pipe the output to another tty. You can do this from a desktop session like so:

Open two terminal emulators.
In the first terminal run the tty command. This command will return the tty device of the terminal emulator (eg. `/dev/pts/1`).
In the second terminal navigate to this repo and run,
 `cargo run -- -d -i=garfanzo > /dev/pts/1`

 To create a window, try this: `window new TITLE T MESSAGE plain 20 20 10 10`

