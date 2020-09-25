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

```

#### Score Boards

```

```

#### Progress Bars

```

```