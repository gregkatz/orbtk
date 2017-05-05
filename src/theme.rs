use orbclient::Color;

const BLACK: Color = Color{ data: 0x000000 | 0xFF000000 };
const SELECT_BLUE: Color = Color{ data: 0x5294E2 | 0xFF000000 };
const BORDER_GREY: Color = Color{ data:0xCFD6E6 | 0xFF000000 };
const WINDOW_GREY: Color = Color{ data:0xF5F6F7 | 0xFF000000 };
const BUTTON_WHITE: Color = Color{ data:0xFBFBFC | 0xFF000000 };
const WHITE: Color = Color{ data:0xFFFFFF | 0xFF000000 };


pub static WINDOW_BACKGROUND: Color = WINDOW_GREY;

pub static LABEL_BACKGROUND: Color = WINDOW_GREY;
pub static LABEL_BORDER: Color = BORDER_GREY;
pub static LABEL_FOREGROUND: Color = BLACK;

pub static BUTTON_BACKGROUND: Color = BUTTON_WHITE;
pub static BUTTON_BG_SELECTION: Color = SELECT_BLUE;
pub static BUTTON_BORDER: Color = BORDER_GREY;
pub static BUTTON_FOREGROUND: Color = BLACK;
pub static BUTTON_FG_SELECTION: Color = WHITE;

pub static ITEM_BACKGROUND: Color = WHITE;
pub static ITEM_BORDER: Color = BORDER_GREY;
pub static ITEM_FOREGROUND: Color = BLACK;
pub static ITEM_SELECTION: Color = SELECT_BLUE;

pub static TEXT_BACKGROUND: Color = WHITE;
pub static TEXT_BORDER: Color = BORDER_GREY;
pub static TEXT_FOREGROUND: Color = BLACK;
pub static TEXT_SELECTION: Color = SELECT_BLUE;
