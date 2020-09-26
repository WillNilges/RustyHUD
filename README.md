# RustyHUD

I've stolen my TUI window manager out of [The Computer Science House ImagineRIT 2020](https://github.com/ComputerScienceHouse/altctrl) project repo and made a crate out of it. My vision is a useful and powerful menu/HUD system for terminal-based games. Yes, you could probably get away with using [cursive](https://github.com/gyscos/cursive) or [tui-rs](https://github.com/fdehau/tui-rs) or something, but I wanted to use what I had already written to try to learn something! The main goal of this is to provide a dead-simple, lightweight, and unobtrusive way to create UI elements overtop of whatever else is in your ncurses session. Ideally, you should be able to toggle this like an overlay on interactive applications.

### Dependencies:

 - `ncurses`
 - `ncurses-dev`

###  Usage:

I recommend you check out my [demo repo](https://github.com/willnilges/rustyhud-demo) to see some examples of how to use this crate.

There are a few big functionalities that you can use:

#### Text Boxes

```
let mut window = WindowData::new()
        .with_id("My Window")
        .with_message("Hello World!")
        .with_position(20, 10)
        .with_dimensions(10, 3)
        .build();

window.open();
```

#### Lists

```
let mut list = WindowData::new()
        .with_id("Sample list")
        .with_content(WindowContent::List)
        .with_items(["Option a".to_string(), "Option b".to_string(), "Option c".to_string()])
        .with_position(12, 3)
        .with_dimensions(10, 3)
        .build();

list.open();
```

#### Score Boards

```
let mut score_board = WindowData::new()
        .with_id("Sample board")
        .with_content(WindowContent::ScoreBoard)
        .with_items(["Character a:10".to_string(), "Character b:9".to_string(), "Character Z-theta-RNQ-B:8".to_string()])
        .with_position(12, 3)
        .with_dimensions(45, 3)
        .build();

score_board.open();
```

#### Progress Bars

```
let mut example_bar = WindowData::new()
        .with_id("Health")
        .with_content(WindowContent::ProgressBar)
        .with_message("20/20")
        .with_ticks("+")
        .with_position(15, 3)
        .with_dimensions(30, 1)
        .build();

example_bar.open();
```