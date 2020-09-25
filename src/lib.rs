use ncurses::*;

// New window struct
#[derive(Clone, Debug)]
pub struct WindowData {
    pub id       : String,
    pub content  : WindowContent,
    pub message  : String,
    pub style    : WindowStyle,
    pub ticks    : Option<String>,
    pub x_pos    : i32,
    pub y_pos    : i32,
    pub width    : i32,
    pub height   : i32,
    pub priority : bool, // Deprecated.
    ncurses_win  : Option<WINDOW>,
}

#[derive(Clone, Debug, Copy)]
pub enum WindowContent {
    Text,
    List,
    ScoreBoard,
    ProgressBar
}

#[derive(Clone, Debug, Copy)]
pub enum WindowStyle {
    Plain,
    Bold,
    Highlight,
    Blink,
    Underline,
}

#[allow(clippy::new_ret_no_self)]
impl WindowData {
    pub fn new() -> WindowDataBuilder {
        WindowDataBuilder {
            id: String::from("Rusty Window"),
            content: WindowContent::Text, // Default to text, because, well, that's ez.
            message: String::from(" "),
            style: WindowStyle::Plain,
            ticks: None,
            x_pos: 1,
            y_pos: 1,
            width: 1,
            height: 1,
        }
    }

    // Opens a new window and keeps track of it in the window HashMap.
    pub fn open(&mut self) {
        // Grab all the data out of the WindowData struct for later use. Cuts down on verbosity.
        // Top left corner of window's (x,y) location. 0,0 is top left of screen.
        // Window ID. What you type to do things to it. Also displayed at the top of the window.
        let mut max_x = 0;
        let mut max_y = 0;
        let start_x;
        let start_y;
        match self.x_pos+self.y_pos {
            -2 => {
                /* Get the screen bounds. */
                getmaxyx(stdscr(), &mut max_y, &mut max_x);
                start_y = max_y / 2;
                start_x = max_x / 2;
            },
            _ => {
                max_x = self.x_pos;
                max_y = self.y_pos;
                start_y = max_y;
                start_x = max_x;
            },
        }
        self.ncurses_win = Some(newwin((self.height)+2, (self.width)+2, start_y, start_x));
        draw_win(&self);
        refresh(); // We're gonna be calling this anyway.
    }

    pub fn mv(&mut self, x: i32, y: i32) {
        self.close();
        self.x_pos = x;
        self.y_pos = y;
        self.open();
        refresh();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.close();
        self.width = width;
        self.height = height;
        self.open();
        refresh();
    }

    pub fn redraw(&mut self) {
        self.close();
        self.open();
        refresh();
    }

    pub fn close(&mut self) {
        let window = self.ncurses_win.unwrap();
        let ch = ' ' as chtype;
        wborder(window, ch, ch, ch, ch, ch, ch, ch, ch);
        wrefresh(window);
        delwin(window);
        refresh();
    }
}

pub struct WindowDataBuilder {
    pub id : String, // The title of the window and what you'll refer to it with
    pub content: WindowContent, // What kind of window it will be
    pub message  : String, // The contents of a window.
    pub style    : WindowStyle,
    pub ticks    : Option<String>,
    pub x_pos    : i32,
    pub y_pos    : i32,
    pub width    : i32,
    pub height   : i32,
}

impl WindowDataBuilder {
    pub fn build(self) -> WindowData {
        WindowData {
            id: self.id,
            content: self.content,
            message: self.message,
            style: self.style,
            ticks: self.ticks,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            width: self.width,
            height: self.height,
            priority: false,
            ncurses_win: None,
        }
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }

    pub fn with_content(mut self, content: WindowContent) -> Self {
        self.content = content;
        self
    }

    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }
    
    pub fn with_style(mut self, style: WindowStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_ticks<S: Into<String>>(mut self, ticks: S) -> Self {
        self.ticks = Some(ticks.into());
        self
    }

    pub fn with_position(mut self, position_x: i32, position_y: i32) -> Self {
        self.x_pos = position_x.into();
        self.y_pos = position_y.into();
        self
    }

    pub fn with_dimensions(mut self, dimension_x: i32, dimension_y: i32) -> Self {
        self.width = dimension_x;
        self.height = dimension_y;
        self
    }
}

pub fn launch() {
    // Setup ncurses.
    initscr();
    raw();

    // Allow for extended keyboard (like F1).
    keypad(stdscr(), true);
    noecho();

    // Invisible cursor.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Log buffer to use for keeping track of command output.
    let mut logbuffer: Vec<String> = Vec::new();
    for _i in 0..5 {
        logbuffer.push(" ".to_string());
    }

    refresh();

    // Get the screen bounds. TODO: Handle resizing.
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
}

// Renders the window using parameters from a WindowData object.
fn draw_win(new_window: &WindowData) {
    // Reduce verbosity.
    let x_loc = new_window.x_pos;
    let y_loc = new_window.y_pos;
    let x_dim = new_window.width;
    let y_dim = new_window.height;
    let name = &new_window.id;
    let message = &new_window.message;
    let style = &new_window.style;
    let ticks = &new_window.ticks;
    let win = &new_window.ncurses_win;
    let mut max_x = 0;
    let mut max_y = 0;
    let start_x;
    let start_y;

    match x_loc+y_loc {
        -2 => {
            /* Get the screen bounds. */
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            start_y = max_y / 2;
            start_x = max_x / 2;
        },
        _ => {
            max_x = x_loc;
            max_y = y_loc;
            start_y = max_y;
            start_x = max_x;
        },
    }

    let mut attribute = A_NORMAL();
    match style {
        WindowStyle::Bold => {
            attribute = A_BOLD();
        },
        WindowStyle::Highlight => {
            attribute = A_STANDOUT();
        },
        WindowStyle::Blink => {
            attribute = A_BLINK();
        },
        WindowStyle::Underline => {
            attribute = A_UNDERLINE();
        },
        _ => {},
    }

    // Match content, then use that to figure out the data.
    match new_window.content {
        WindowContent::Text => { // Display whatever text you need in a normal, window wrapping fashion.
            attron(attribute);
            if message.len() > (x_dim as usize) {
                let real_x_dim = x_dim as usize;
                for i in 0..message.len(){
                    if i == 0 {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[0..real_x_dim]);
                    } else if real_x_dim*(i+1) >= message.len() {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[real_x_dim*(i)..]);
                        break;
                    } else {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
                    }
                }
            } else {
                mvprintw(start_y+1, start_x+1, &message);
            }
            attroff(attribute);
        },
        WindowContent::List => { // Display a list of items or options
            let list_data = message.split('|').collect::<Vec<&str>>();
            attron(A_UNDERLINE());
            for i in 0..list_data.len() {
                for j in 0..x_dim {
                    mvprintw(start_y+1+(i as i32), start_x+1+(j as i32), " "); // Print the underline to separate list items.
                }
                attron(attribute);
                mvprintw(start_y+1+(i as i32), start_x+1, &list_data[i]);
                attroff(attribute);
            }
            attroff(A_UNDERLINE());
        },
        WindowContent::ScoreBoard => { // Like a list, but you can pair numbers with it. Unsorted.
            let list_data = message.split('|').collect::<Vec<&str>>();
            attron(A_UNDERLINE());
            for i in 0..list_data.len() {
                for j in 0..x_dim {
                    mvprintw(start_y+1+(i as i32), start_x+1+(j as i32), " ");
                }
                let item_metric = &list_data[i].split(':').collect::<Vec<&str>>();
                if item_metric.len() >= 1 { // I guess I can display a name with no score on the scoreboard.
                    attron(attribute);
                    mvprintw(start_y+1+(i as i32), start_x+1, item_metric[0]);
                    attroff(attribute);
                }
                attron(A_BOLD());
                if item_metric.len() == 2 { // The damn thing should be at most two values
                    attron(attribute);
                    mvprintw(start_y+1+(i as i32), start_x+x_dim-3, item_metric[1]);
                    attroff(attribute);
                }
                attroff(A_BOLD());
            }
            attroff(A_UNDERLINE());
        },
        WindowContent::ProgressBar => {
            // Display a bar of some sort in a window.
            // (Window heights of 1 work best).
            
            let mut pb_bg = A_STANDOUT();
            let pb_ch = match ticks {
                Some(character) => character,
                _ => "#",
            };

            match style {
                WindowStyle::Plain => pb_bg = A_NORMAL(),
                WindowStyle::Blink => pb_bg = A_BLINK(),
                _ => {},
            }

            let metrics = message.split('/').collect::<Vec<&str>>();
            let lower = metrics[0].parse::<f32>().unwrap();
            let upper = metrics[1].parse::<f32>().unwrap();
            let absolute_progress = ((lower/upper)*(x_dim as f32)) as i32; // How far across the window the bar is
            // if pb_style[0] == "flash" { attron(A_STANDOUT()); }
            attron(pb_bg); // Solid bar style. TODO: Make more styles?
            for i in 0..absolute_progress {
                mvprintw(start_y+1, start_x+1+(i as i32), pb_ch);
            }
            attroff(pb_bg);
            // if pb_style[0] == "flash" { attroff(A_STANDOUT()); }
            // Print the value
            attron(A_BOLD());
            let progress_string = format!("|{}/{}|", lower, upper);
            mvprintw(start_y+y_dim+1, start_x+x_dim-1-message.len() as i32, &progress_string);
            attroff(A_BOLD());
        },
    }
    box_(win, 0, 0);
    wrefresh(win);
    attron(A_BOLD());
    let title = format!("|{}|", name);
    mvprintw(start_y, start_x+1, &title);
    attroff(A_BOLD());
}
