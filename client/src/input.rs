#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum Input {
    KeyboardKey1,
    KeyboardKey2,
    KeyboardKey3,
    KeyboardKey4,
    KeyboardKey5,
    KeyboardKey6,
    KeyboardKey7,
    KeyboardKey8,
    KeyboardKey9,
    KeyboardKey0,
    KeyboardA,
    KeyboardB,
    KeyboardC,
    KeyboardD,
    KeyboardE,
    KeyboardF,
    KeyboardG,
    KeyboardH,
    KeyboardI,
    KeyboardJ,
    KeyboardK,
    KeyboardL,
    KeyboardM,
    KeyboardN,
    KeyboardO,
    KeyboardP,
    KeyboardQ,
    KeyboardR,
    KeyboardS,
    KeyboardT,
    KeyboardU,
    KeyboardV,
    KeyboardW,
    KeyboardX,
    KeyboardY,
    KeyboardZ,
    KeyboardEscape,
    KeyboardF1,
    KeyboardF2,
    KeyboardF3,
    KeyboardF4,
    KeyboardF5,
    KeyboardF6,
    KeyboardF7,
    KeyboardF8,
    KeyboardF9,
    KeyboardF10,
    KeyboardF11,
    KeyboardF12,
    KeyboardF13,
    KeyboardF14,
    KeyboardF15,
    KeyboardF16,
    KeyboardF17,
    KeyboardF18,
    KeyboardF19,
    KeyboardF20,
    KeyboardF21,
    KeyboardF22,
    KeyboardF23,
    KeyboardF24,
    KeyboardSnapshot,
    KeyboardScroll,
    KeyboardPause,
    KeyboardInsert,
    KeyboardHome,
    KeyboardDelete,
    KeyboardEnd,
    KeyboardPageDown,
    KeyboardPageUp,
    KeyboardLeft,
    KeyboardUp,
    KeyboardRight,
    KeyboardDown,
    KeyboardBack,
    KeyboardReturn,
    KeyboardSpace,
    KeyboardCompose,
    KeyboardCaret,
    KeyboardNumlock,
    KeyboardNumpad0,
    KeyboardNumpad1,
    KeyboardNumpad2,
    KeyboardNumpad3,
    KeyboardNumpad4,
    KeyboardNumpad5,
    KeyboardNumpad6,
    KeyboardNumpad7,
    KeyboardNumpad8,
    KeyboardNumpad9,
    KeyboardNumpadAdd,
    KeyboardNumpadDivide,
    KeyboardNumpadDecimal,
    KeyboardNumpadComma,
    KeyboardNumpadEnter,
    KeyboardNumpadEquals,
    KeyboardNumpadMultiply,
    KeyboardNumpadSubtract,
    KeyboardAbntC1,
    KeyboardAbntC2,
    KeyboardApostrophe,
    KeyboardApps,
    KeyboardAsterisk,
    KeyboardAt,
    KeyboardAx,
    KeyboardBackslash,
    KeyboardCalculator,
    KeyboardCapital,
    KeyboardColon,
    KeyboardComma,
    KeyboardConvert,
    KeyboardEquals,
    KeyboardGrave,
    KeyboardKana,
    KeyboardKanji,
    KeyboardLAlt,
    KeyboardLBracket,
    KeyboardLControl,
    KeyboardLShift,
    KeyboardLWin,
    KeyboardMail,
    KeyboardMediaSelect,
    KeyboardMediaStop,
    KeyboardMinus,
    KeyboardMute,
    KeyboardMyComputer,
    KeyboardNavigateForward,
    KeyboardNavigateBackward,
    KeyboardNextTrack,
    KeyboardNoConvert,
    KeyboardOEM102,
    KeyboardPeriod,
    KeyboardPlayPause,
    KeyboardPlus,
    KeyboardPower,
    KeyboardPrevTrack,
    KeyboardRAlt,
    KeyboardRBracket,
    KeyboardRControl,
    KeyboardRShift,
    KeyboardRWin,
    KeyboardSemicolon,
    KeyboardSlash,
    KeyboardSleep,
    KeyboardStop,
    KeyboardSysrq,
    KeyboardTab,
    KeyboardUnderline,
    KeyboardUnlabeled,
    KeyboardVolumeDown,
    KeyboardVolumeUp,
    KeyboardWake,
    KeyboardWebBack,
    KeyboardWebFavorites,
    KeyboardWebForward,
    KeyboardWebHome,
    KeyboardWebRefresh,
    KeyboardWebSearch,
    KeyboardWebStop,
    KeyboardYen,
    KeyboardCopy,
    KeyboardPaste,
    KeyboardCut,
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

pub fn translate_input_to_keycode(input: Input) -> Option<ggez::input::keyboard::KeyCode> {
    match input {
        Input::KeyboardKey1 => Some(ggez::input::keyboard::KeyCode::Key1),
        Input::KeyboardKey2 => Some(ggez::input::keyboard::KeyCode::Key2),
        Input::KeyboardKey3 => Some(ggez::input::keyboard::KeyCode::Key3),
        Input::KeyboardKey4 => Some(ggez::input::keyboard::KeyCode::Key4),
        Input::KeyboardKey5 => Some(ggez::input::keyboard::KeyCode::Key5),
        Input::KeyboardKey6 => Some(ggez::input::keyboard::KeyCode::Key6),
        Input::KeyboardKey7 => Some(ggez::input::keyboard::KeyCode::Key7),
        Input::KeyboardKey8 => Some(ggez::input::keyboard::KeyCode::Key8),
        Input::KeyboardKey9 => Some(ggez::input::keyboard::KeyCode::Key9),
        Input::KeyboardKey0 => Some(ggez::input::keyboard::KeyCode::Key0),
        Input::KeyboardA => Some(ggez::input::keyboard::KeyCode::A),
        Input::KeyboardB => Some(ggez::input::keyboard::KeyCode::B),
        Input::KeyboardC => Some(ggez::input::keyboard::KeyCode::C),
        Input::KeyboardD => Some(ggez::input::keyboard::KeyCode::D),
        Input::KeyboardE => Some(ggez::input::keyboard::KeyCode::E),
        Input::KeyboardF => Some(ggez::input::keyboard::KeyCode::F),
        Input::KeyboardG => Some(ggez::input::keyboard::KeyCode::G),
        Input::KeyboardH => Some(ggez::input::keyboard::KeyCode::H),
        Input::KeyboardI => Some(ggez::input::keyboard::KeyCode::I),
        Input::KeyboardJ => Some(ggez::input::keyboard::KeyCode::J),
        Input::KeyboardK => Some(ggez::input::keyboard::KeyCode::K),
        Input::KeyboardL => Some(ggez::input::keyboard::KeyCode::L),
        Input::KeyboardM => Some(ggez::input::keyboard::KeyCode::M),
        Input::KeyboardN => Some(ggez::input::keyboard::KeyCode::N),
        Input::KeyboardO => Some(ggez::input::keyboard::KeyCode::O),
        Input::KeyboardP => Some(ggez::input::keyboard::KeyCode::P),
        Input::KeyboardQ => Some(ggez::input::keyboard::KeyCode::Q),
        Input::KeyboardR => Some(ggez::input::keyboard::KeyCode::R),
        Input::KeyboardS => Some(ggez::input::keyboard::KeyCode::S),
        Input::KeyboardT => Some(ggez::input::keyboard::KeyCode::T),
        Input::KeyboardU => Some(ggez::input::keyboard::KeyCode::U),
        Input::KeyboardV => Some(ggez::input::keyboard::KeyCode::V),
        Input::KeyboardW => Some(ggez::input::keyboard::KeyCode::W),
        Input::KeyboardX => Some(ggez::input::keyboard::KeyCode::X),
        Input::KeyboardY => Some(ggez::input::keyboard::KeyCode::Y),
        Input::KeyboardZ => Some(ggez::input::keyboard::KeyCode::Z),
        Input::KeyboardEscape => Some(ggez::input::keyboard::KeyCode::Escape),
        Input::KeyboardF1 => Some(ggez::input::keyboard::KeyCode::F1),
        Input::KeyboardF2 => Some(ggez::input::keyboard::KeyCode::F2),
        Input::KeyboardF3 => Some(ggez::input::keyboard::KeyCode::F3),
        Input::KeyboardF4 => Some(ggez::input::keyboard::KeyCode::F4),
        Input::KeyboardF5 => Some(ggez::input::keyboard::KeyCode::F5),
        Input::KeyboardF6 => Some(ggez::input::keyboard::KeyCode::F6),
        Input::KeyboardF7 => Some(ggez::input::keyboard::KeyCode::F7),
        Input::KeyboardF8 => Some(ggez::input::keyboard::KeyCode::F8),
        Input::KeyboardF9 => Some(ggez::input::keyboard::KeyCode::F9),
        Input::KeyboardF10 => Some(ggez::input::keyboard::KeyCode::F10),
        Input::KeyboardF11 => Some(ggez::input::keyboard::KeyCode::F11),
        Input::KeyboardF12 => Some(ggez::input::keyboard::KeyCode::F12),
        Input::KeyboardF13 => Some(ggez::input::keyboard::KeyCode::F13),
        Input::KeyboardF14 => Some(ggez::input::keyboard::KeyCode::F14),
        Input::KeyboardF15 => Some(ggez::input::keyboard::KeyCode::F15),
        Input::KeyboardF16 => Some(ggez::input::keyboard::KeyCode::F16),
        Input::KeyboardF17 => Some(ggez::input::keyboard::KeyCode::F17),
        Input::KeyboardF18 => Some(ggez::input::keyboard::KeyCode::F18),
        Input::KeyboardF19 => Some(ggez::input::keyboard::KeyCode::F19),
        Input::KeyboardF20 => Some(ggez::input::keyboard::KeyCode::F20),
        Input::KeyboardF21 => Some(ggez::input::keyboard::KeyCode::F21),
        Input::KeyboardF22 => Some(ggez::input::keyboard::KeyCode::F22),
        Input::KeyboardF23 => Some(ggez::input::keyboard::KeyCode::F23),
        Input::KeyboardF24 => Some(ggez::input::keyboard::KeyCode::F24),
        Input::KeyboardSnapshot => Some(ggez::input::keyboard::KeyCode::Snapshot),
        Input::KeyboardScroll => Some(ggez::input::keyboard::KeyCode::Scroll),
        Input::KeyboardPause => Some(ggez::input::keyboard::KeyCode::Pause),
        Input::KeyboardInsert => Some(ggez::input::keyboard::KeyCode::Insert),
        Input::KeyboardHome => Some(ggez::input::keyboard::KeyCode::Home),
        Input::KeyboardDelete => Some(ggez::input::keyboard::KeyCode::Delete),
        Input::KeyboardEnd => Some(ggez::input::keyboard::KeyCode::End),
        Input::KeyboardPageDown => Some(ggez::input::keyboard::KeyCode::PageDown),
        Input::KeyboardPageUp => Some(ggez::input::keyboard::KeyCode::PageUp),
        Input::KeyboardLeft => Some(ggez::input::keyboard::KeyCode::Left),
        Input::KeyboardUp => Some(ggez::input::keyboard::KeyCode::Up),
        Input::KeyboardRight => Some(ggez::input::keyboard::KeyCode::Right),
        Input::KeyboardDown => Some(ggez::input::keyboard::KeyCode::Down),
        Input::KeyboardBack => Some(ggez::input::keyboard::KeyCode::Back),
        Input::KeyboardReturn => Some(ggez::input::keyboard::KeyCode::Return),
        Input::KeyboardSpace => Some(ggez::input::keyboard::KeyCode::Space),
        Input::KeyboardCompose => Some(ggez::input::keyboard::KeyCode::Compose),
        Input::KeyboardCaret => Some(ggez::input::keyboard::KeyCode::Caret),
        Input::KeyboardNumlock => Some(ggez::input::keyboard::KeyCode::Numlock),
        Input::KeyboardNumpad0 => Some(ggez::input::keyboard::KeyCode::Numpad0),
        Input::KeyboardNumpad1 => Some(ggez::input::keyboard::KeyCode::Numpad1),
        Input::KeyboardNumpad2 => Some(ggez::input::keyboard::KeyCode::Numpad2),
        Input::KeyboardNumpad3 => Some(ggez::input::keyboard::KeyCode::Numpad3),
        Input::KeyboardNumpad4 => Some(ggez::input::keyboard::KeyCode::Numpad4),
        Input::KeyboardNumpad5 => Some(ggez::input::keyboard::KeyCode::Numpad5),
        Input::KeyboardNumpad6 => Some(ggez::input::keyboard::KeyCode::Numpad6),
        Input::KeyboardNumpad7 => Some(ggez::input::keyboard::KeyCode::Numpad7),
        Input::KeyboardNumpad8 => Some(ggez::input::keyboard::KeyCode::Numpad8),
        Input::KeyboardNumpad9 => Some(ggez::input::keyboard::KeyCode::Numpad9),
        Input::KeyboardNumpadAdd => Some(ggez::input::keyboard::KeyCode::NumpadAdd),
        Input::KeyboardNumpadDivide => Some(ggez::input::keyboard::KeyCode::NumpadDivide),
        Input::KeyboardNumpadDecimal => Some(ggez::input::keyboard::KeyCode::NumpadDecimal),
        Input::KeyboardNumpadComma => Some(ggez::input::keyboard::KeyCode::NumpadComma),
        Input::KeyboardNumpadEnter => Some(ggez::input::keyboard::KeyCode::NumpadEnter),
        Input::KeyboardNumpadEquals => Some(ggez::input::keyboard::KeyCode::NumpadEquals),
        Input::KeyboardNumpadMultiply => Some(ggez::input::keyboard::KeyCode::NumpadMultiply),
        Input::KeyboardNumpadSubtract => Some(ggez::input::keyboard::KeyCode::NumpadSubtract),
        Input::KeyboardAbntC1 => Some(ggez::input::keyboard::KeyCode::AbntC1),
        Input::KeyboardAbntC2 => Some(ggez::input::keyboard::KeyCode::AbntC2),
        Input::KeyboardApostrophe => Some(ggez::input::keyboard::KeyCode::Apostrophe),
        Input::KeyboardApps => Some(ggez::input::keyboard::KeyCode::Apps),
        Input::KeyboardAsterisk => Some(ggez::input::keyboard::KeyCode::Asterisk),
        Input::KeyboardAt => Some(ggez::input::keyboard::KeyCode::At),
        Input::KeyboardAx => Some(ggez::input::keyboard::KeyCode::Ax),
        Input::KeyboardBackslash => Some(ggez::input::keyboard::KeyCode::Backslash),
        Input::KeyboardCalculator => Some(ggez::input::keyboard::KeyCode::Calculator),
        Input::KeyboardCapital => Some(ggez::input::keyboard::KeyCode::Capital),
        Input::KeyboardColon => Some(ggez::input::keyboard::KeyCode::Colon),
        Input::KeyboardComma => Some(ggez::input::keyboard::KeyCode::Comma),
        Input::KeyboardConvert => Some(ggez::input::keyboard::KeyCode::Convert),
        Input::KeyboardEquals => Some(ggez::input::keyboard::KeyCode::Equals),
        Input::KeyboardGrave => Some(ggez::input::keyboard::KeyCode::Grave),
        Input::KeyboardKana => Some(ggez::input::keyboard::KeyCode::Kana),
        Input::KeyboardKanji => Some(ggez::input::keyboard::KeyCode::Kanji),
        Input::KeyboardLAlt => Some(ggez::input::keyboard::KeyCode::LAlt),
        Input::KeyboardLBracket => Some(ggez::input::keyboard::KeyCode::LBracket),
        Input::KeyboardLControl => Some(ggez::input::keyboard::KeyCode::LControl),
        Input::KeyboardLShift => Some(ggez::input::keyboard::KeyCode::LShift),
        Input::KeyboardLWin => Some(ggez::input::keyboard::KeyCode::LWin),
        Input::KeyboardMail => Some(ggez::input::keyboard::KeyCode::Mail),
        Input::KeyboardMediaSelect => Some(ggez::input::keyboard::KeyCode::MediaSelect),
        Input::KeyboardMediaStop => Some(ggez::input::keyboard::KeyCode::MediaStop),
        Input::KeyboardMinus => Some(ggez::input::keyboard::KeyCode::Minus),
        Input::KeyboardMute => Some(ggez::input::keyboard::KeyCode::Mute),
        Input::KeyboardMyComputer => Some(ggez::input::keyboard::KeyCode::MyComputer),
        Input::KeyboardNavigateForward => Some(ggez::input::keyboard::KeyCode::NavigateForward),
        Input::KeyboardNavigateBackward => Some(ggez::input::keyboard::KeyCode::NavigateBackward),
        Input::KeyboardNextTrack => Some(ggez::input::keyboard::KeyCode::NextTrack),
        Input::KeyboardNoConvert => Some(ggez::input::keyboard::KeyCode::NoConvert),
        Input::KeyboardOEM102 => Some(ggez::input::keyboard::KeyCode::OEM102),
        Input::KeyboardPeriod => Some(ggez::input::keyboard::KeyCode::Period),
        Input::KeyboardPlayPause => Some(ggez::input::keyboard::KeyCode::PlayPause),
        Input::KeyboardPlus => Some(ggez::input::keyboard::KeyCode::Plus),
        Input::KeyboardPower => Some(ggez::input::keyboard::KeyCode::Power),
        Input::KeyboardPrevTrack => Some(ggez::input::keyboard::KeyCode::PrevTrack),
        Input::KeyboardRAlt => Some(ggez::input::keyboard::KeyCode::RAlt),
        Input::KeyboardRBracket => Some(ggez::input::keyboard::KeyCode::RBracket),
        Input::KeyboardRControl => Some(ggez::input::keyboard::KeyCode::RControl),
        Input::KeyboardRShift => Some(ggez::input::keyboard::KeyCode::RShift),
        Input::KeyboardRWin => Some(ggez::input::keyboard::KeyCode::RWin),
        Input::KeyboardSemicolon => Some(ggez::input::keyboard::KeyCode::Semicolon),
        Input::KeyboardSlash => Some(ggez::input::keyboard::KeyCode::Slash),
        Input::KeyboardSleep => Some(ggez::input::keyboard::KeyCode::Sleep),
        Input::KeyboardStop => Some(ggez::input::keyboard::KeyCode::Stop),
        Input::KeyboardSysrq => Some(ggez::input::keyboard::KeyCode::Sysrq),
        Input::KeyboardTab => Some(ggez::input::keyboard::KeyCode::Tab),
        Input::KeyboardUnderline => Some(ggez::input::keyboard::KeyCode::Underline),
        Input::KeyboardUnlabeled => Some(ggez::input::keyboard::KeyCode::Unlabeled),
        Input::KeyboardVolumeDown => Some(ggez::input::keyboard::KeyCode::VolumeDown),
        Input::KeyboardVolumeUp => Some(ggez::input::keyboard::KeyCode::VolumeUp),
        Input::KeyboardWake => Some(ggez::input::keyboard::KeyCode::Wake),
        Input::KeyboardWebBack => Some(ggez::input::keyboard::KeyCode::WebBack),
        Input::KeyboardWebFavorites => Some(ggez::input::keyboard::KeyCode::WebFavorites),
        Input::KeyboardWebForward => Some(ggez::input::keyboard::KeyCode::WebForward),
        Input::KeyboardWebHome => Some(ggez::input::keyboard::KeyCode::WebHome),
        Input::KeyboardWebRefresh => Some(ggez::input::keyboard::KeyCode::WebRefresh),
        Input::KeyboardWebSearch => Some(ggez::input::keyboard::KeyCode::WebSearch),
        Input::KeyboardWebStop => Some(ggez::input::keyboard::KeyCode::WebStop),
        Input::KeyboardYen => Some(ggez::input::keyboard::KeyCode::Yen),
        Input::KeyboardCopy => Some(ggez::input::keyboard::KeyCode::Copy),
        Input::KeyboardPaste => Some(ggez::input::keyboard::KeyCode::Paste),
        Input::KeyboardCut => Some(ggez::input::keyboard::KeyCode::Cut),
        _ => None, // mouse button
    }
}

pub fn translate_input_to_mousebutton(input: Input) -> Option<ggez::input::mouse::MouseButton> {
    match input {
        Input::MouseLeft => Some(ggez::input::mouse::MouseButton::Left),
        Input::MouseRight => Some(ggez::input::mouse::MouseButton::Right),
        Input::MouseMiddle => Some(ggez::input::mouse::MouseButton::Middle),
        _ => None, // keyboard button
    }
}

pub fn translate_mousebutton_to_input(button: ggez::input::mouse::MouseButton) -> Option<Input> {
    match button {
        ggez::input::mouse::MouseButton::Left => Some(Input::MouseLeft),
        ggez::input::mouse::MouseButton::Right => Some(Input::MouseRight),
        ggez::input::mouse::MouseButton::Middle => Some(Input::MouseMiddle),
        _ => {
            error!("The mouse button {button:?} could not be translated into a input");
            None
        } // this is when the user's mouse have ton of buttons, i'll take care of it later
    }
}

pub fn translate_keycode_to_input(code: ggez::input::keyboard::KeyCode) -> Option<Input> {
    match code {
        ggez::input::keyboard::KeyCode::Key1 => Some(Input::KeyboardKey1),
        ggez::input::keyboard::KeyCode::Key2 => Some(Input::KeyboardKey2),
        ggez::input::keyboard::KeyCode::Key3 => Some(Input::KeyboardKey3),
        ggez::input::keyboard::KeyCode::Key4 => Some(Input::KeyboardKey4),
        ggez::input::keyboard::KeyCode::Key5 => Some(Input::KeyboardKey5),
        ggez::input::keyboard::KeyCode::Key6 => Some(Input::KeyboardKey6),
        ggez::input::keyboard::KeyCode::Key7 => Some(Input::KeyboardKey7),
        ggez::input::keyboard::KeyCode::Key8 => Some(Input::KeyboardKey8),
        ggez::input::keyboard::KeyCode::Key9 => Some(Input::KeyboardKey9),
        ggez::input::keyboard::KeyCode::Key0 => Some(Input::KeyboardKey0),
        ggez::input::keyboard::KeyCode::A => Some(Input::KeyboardA),
        ggez::input::keyboard::KeyCode::B => Some(Input::KeyboardB),
        ggez::input::keyboard::KeyCode::C => Some(Input::KeyboardC),
        ggez::input::keyboard::KeyCode::D => Some(Input::KeyboardD),
        ggez::input::keyboard::KeyCode::E => Some(Input::KeyboardE),
        ggez::input::keyboard::KeyCode::F => Some(Input::KeyboardF),
        ggez::input::keyboard::KeyCode::G => Some(Input::KeyboardG),
        ggez::input::keyboard::KeyCode::H => Some(Input::KeyboardH),
        ggez::input::keyboard::KeyCode::I => Some(Input::KeyboardI),
        ggez::input::keyboard::KeyCode::J => Some(Input::KeyboardJ),
        ggez::input::keyboard::KeyCode::K => Some(Input::KeyboardK),
        ggez::input::keyboard::KeyCode::L => Some(Input::KeyboardL),
        ggez::input::keyboard::KeyCode::M => Some(Input::KeyboardM),
        ggez::input::keyboard::KeyCode::N => Some(Input::KeyboardN),
        ggez::input::keyboard::KeyCode::O => Some(Input::KeyboardO),
        ggez::input::keyboard::KeyCode::P => Some(Input::KeyboardP),
        ggez::input::keyboard::KeyCode::Q => Some(Input::KeyboardQ),
        ggez::input::keyboard::KeyCode::R => Some(Input::KeyboardR),
        ggez::input::keyboard::KeyCode::S => Some(Input::KeyboardS),
        ggez::input::keyboard::KeyCode::T => Some(Input::KeyboardT),
        ggez::input::keyboard::KeyCode::U => Some(Input::KeyboardU),
        ggez::input::keyboard::KeyCode::V => Some(Input::KeyboardV),
        ggez::input::keyboard::KeyCode::W => Some(Input::KeyboardW),
        ggez::input::keyboard::KeyCode::X => Some(Input::KeyboardX),
        ggez::input::keyboard::KeyCode::Y => Some(Input::KeyboardY),
        ggez::input::keyboard::KeyCode::Z => Some(Input::KeyboardZ),
        ggez::input::keyboard::KeyCode::Escape => Some(Input::KeyboardEscape),
        ggez::input::keyboard::KeyCode::F1 => Some(Input::KeyboardF1),
        ggez::input::keyboard::KeyCode::F2 => Some(Input::KeyboardF2),
        ggez::input::keyboard::KeyCode::F3 => Some(Input::KeyboardF3),
        ggez::input::keyboard::KeyCode::F4 => Some(Input::KeyboardF4),
        ggez::input::keyboard::KeyCode::F5 => Some(Input::KeyboardF5),
        ggez::input::keyboard::KeyCode::F6 => Some(Input::KeyboardF6),
        ggez::input::keyboard::KeyCode::F7 => Some(Input::KeyboardF7),
        ggez::input::keyboard::KeyCode::F8 => Some(Input::KeyboardF8),
        ggez::input::keyboard::KeyCode::F9 => Some(Input::KeyboardF9),
        ggez::input::keyboard::KeyCode::F10 => Some(Input::KeyboardF10),
        ggez::input::keyboard::KeyCode::F11 => Some(Input::KeyboardF11),
        ggez::input::keyboard::KeyCode::F12 => Some(Input::KeyboardF12),
        ggez::input::keyboard::KeyCode::F13 => Some(Input::KeyboardF13),
        ggez::input::keyboard::KeyCode::F14 => Some(Input::KeyboardF14),
        ggez::input::keyboard::KeyCode::F15 => Some(Input::KeyboardF15),
        ggez::input::keyboard::KeyCode::F16 => Some(Input::KeyboardF16),
        ggez::input::keyboard::KeyCode::F17 => Some(Input::KeyboardF17),
        ggez::input::keyboard::KeyCode::F18 => Some(Input::KeyboardF18),
        ggez::input::keyboard::KeyCode::F19 => Some(Input::KeyboardF19),
        ggez::input::keyboard::KeyCode::F20 => Some(Input::KeyboardF20),
        ggez::input::keyboard::KeyCode::F21 => Some(Input::KeyboardF21),
        ggez::input::keyboard::KeyCode::F22 => Some(Input::KeyboardF22),
        ggez::input::keyboard::KeyCode::F23 => Some(Input::KeyboardF23),
        ggez::input::keyboard::KeyCode::F24 => Some(Input::KeyboardF24),
        ggez::input::keyboard::KeyCode::Snapshot => Some(Input::KeyboardSnapshot),
        ggez::input::keyboard::KeyCode::Scroll => Some(Input::KeyboardScroll),
        ggez::input::keyboard::KeyCode::Pause => Some(Input::KeyboardPause),
        ggez::input::keyboard::KeyCode::Insert => Some(Input::KeyboardInsert),
        ggez::input::keyboard::KeyCode::Home => Some(Input::KeyboardHome),
        ggez::input::keyboard::KeyCode::Delete => Some(Input::KeyboardDelete),
        ggez::input::keyboard::KeyCode::End => Some(Input::KeyboardEnd),
        ggez::input::keyboard::KeyCode::PageDown => Some(Input::KeyboardPageDown),
        ggez::input::keyboard::KeyCode::PageUp => Some(Input::KeyboardPageUp),
        ggez::input::keyboard::KeyCode::Left => Some(Input::KeyboardLeft),
        ggez::input::keyboard::KeyCode::Up => Some(Input::KeyboardUp),
        ggez::input::keyboard::KeyCode::Right => Some(Input::KeyboardRight),
        ggez::input::keyboard::KeyCode::Down => Some(Input::KeyboardDown),
        ggez::input::keyboard::KeyCode::Back => Some(Input::KeyboardBack),
        ggez::input::keyboard::KeyCode::Return => Some(Input::KeyboardReturn),
        ggez::input::keyboard::KeyCode::Space => Some(Input::KeyboardSpace),
        ggez::input::keyboard::KeyCode::Compose => Some(Input::KeyboardCompose),
        ggez::input::keyboard::KeyCode::Caret => Some(Input::KeyboardCaret),
        ggez::input::keyboard::KeyCode::Numlock => Some(Input::KeyboardNumlock),
        ggez::input::keyboard::KeyCode::Numpad0 => Some(Input::KeyboardNumpad0),
        ggez::input::keyboard::KeyCode::Numpad1 => Some(Input::KeyboardNumpad1),
        ggez::input::keyboard::KeyCode::Numpad2 => Some(Input::KeyboardNumpad2),
        ggez::input::keyboard::KeyCode::Numpad3 => Some(Input::KeyboardNumpad3),
        ggez::input::keyboard::KeyCode::Numpad4 => Some(Input::KeyboardNumpad4),
        ggez::input::keyboard::KeyCode::Numpad5 => Some(Input::KeyboardNumpad5),
        ggez::input::keyboard::KeyCode::Numpad6 => Some(Input::KeyboardNumpad6),
        ggez::input::keyboard::KeyCode::Numpad7 => Some(Input::KeyboardNumpad7),
        ggez::input::keyboard::KeyCode::Numpad8 => Some(Input::KeyboardNumpad8),
        ggez::input::keyboard::KeyCode::Numpad9 => Some(Input::KeyboardNumpad9),
        ggez::input::keyboard::KeyCode::NumpadAdd => Some(Input::KeyboardNumpadAdd),
        ggez::input::keyboard::KeyCode::NumpadDivide => Some(Input::KeyboardNumpadDivide),
        ggez::input::keyboard::KeyCode::NumpadDecimal => Some(Input::KeyboardNumpadDecimal),
        ggez::input::keyboard::KeyCode::NumpadComma => Some(Input::KeyboardNumpadComma),
        ggez::input::keyboard::KeyCode::NumpadEnter => Some(Input::KeyboardNumpadEnter),
        ggez::input::keyboard::KeyCode::NumpadEquals => Some(Input::KeyboardNumpadEquals),
        ggez::input::keyboard::KeyCode::NumpadMultiply => Some(Input::KeyboardNumpadMultiply),
        ggez::input::keyboard::KeyCode::NumpadSubtract => Some(Input::KeyboardNumpadSubtract),
        ggez::input::keyboard::KeyCode::AbntC1 => Some(Input::KeyboardAbntC1),
        ggez::input::keyboard::KeyCode::AbntC2 => Some(Input::KeyboardAbntC2),
        ggez::input::keyboard::KeyCode::Apostrophe => Some(Input::KeyboardApostrophe),
        ggez::input::keyboard::KeyCode::Apps => Some(Input::KeyboardApps),
        ggez::input::keyboard::KeyCode::Asterisk => Some(Input::KeyboardAsterisk),
        ggez::input::keyboard::KeyCode::At => Some(Input::KeyboardAt),
        ggez::input::keyboard::KeyCode::Ax => Some(Input::KeyboardAx),
        ggez::input::keyboard::KeyCode::Backslash => Some(Input::KeyboardBackslash),
        ggez::input::keyboard::KeyCode::Calculator => Some(Input::KeyboardCalculator),
        ggez::input::keyboard::KeyCode::Capital => Some(Input::KeyboardCapital),
        ggez::input::keyboard::KeyCode::Colon => Some(Input::KeyboardColon),
        ggez::input::keyboard::KeyCode::Comma => Some(Input::KeyboardComma),
        ggez::input::keyboard::KeyCode::Convert => Some(Input::KeyboardConvert),
        ggez::input::keyboard::KeyCode::Equals => Some(Input::KeyboardEquals),
        ggez::input::keyboard::KeyCode::Grave => Some(Input::KeyboardGrave),
        ggez::input::keyboard::KeyCode::Kana => Some(Input::KeyboardKana),
        ggez::input::keyboard::KeyCode::Kanji => Some(Input::KeyboardKanji),
        ggez::input::keyboard::KeyCode::LAlt => Some(Input::KeyboardLAlt),
        ggez::input::keyboard::KeyCode::LBracket => Some(Input::KeyboardLBracket),
        ggez::input::keyboard::KeyCode::LControl => Some(Input::KeyboardLControl),
        ggez::input::keyboard::KeyCode::LShift => Some(Input::KeyboardLShift),
        ggez::input::keyboard::KeyCode::LWin => Some(Input::KeyboardLWin),
        ggez::input::keyboard::KeyCode::Mail => Some(Input::KeyboardMail),
        ggez::input::keyboard::KeyCode::MediaSelect => Some(Input::KeyboardMediaSelect),
        ggez::input::keyboard::KeyCode::MediaStop => Some(Input::KeyboardMediaStop),
        ggez::input::keyboard::KeyCode::Minus => Some(Input::KeyboardMinus),
        ggez::input::keyboard::KeyCode::Mute => Some(Input::KeyboardMute),
        ggez::input::keyboard::KeyCode::MyComputer => Some(Input::KeyboardMyComputer),
        ggez::input::keyboard::KeyCode::NavigateForward => Some(Input::KeyboardNavigateForward),
        ggez::input::keyboard::KeyCode::NavigateBackward => Some(Input::KeyboardNavigateBackward),
        ggez::input::keyboard::KeyCode::NextTrack => Some(Input::KeyboardNextTrack),
        ggez::input::keyboard::KeyCode::NoConvert => Some(Input::KeyboardNoConvert),
        ggez::input::keyboard::KeyCode::OEM102 => Some(Input::KeyboardOEM102),
        ggez::input::keyboard::KeyCode::Period => Some(Input::KeyboardPeriod),
        ggez::input::keyboard::KeyCode::PlayPause => Some(Input::KeyboardPlayPause),
        ggez::input::keyboard::KeyCode::Plus => Some(Input::KeyboardPlus),
        ggez::input::keyboard::KeyCode::Power => Some(Input::KeyboardPower),
        ggez::input::keyboard::KeyCode::PrevTrack => Some(Input::KeyboardPrevTrack),
        ggez::input::keyboard::KeyCode::RAlt => Some(Input::KeyboardRAlt),
        ggez::input::keyboard::KeyCode::RBracket => Some(Input::KeyboardRBracket),
        ggez::input::keyboard::KeyCode::RControl => Some(Input::KeyboardRControl),
        ggez::input::keyboard::KeyCode::RShift => Some(Input::KeyboardRShift),
        ggez::input::keyboard::KeyCode::RWin => Some(Input::KeyboardRWin),
        ggez::input::keyboard::KeyCode::Semicolon => Some(Input::KeyboardSemicolon),
        ggez::input::keyboard::KeyCode::Slash => Some(Input::KeyboardSlash),
        ggez::input::keyboard::KeyCode::Sleep => Some(Input::KeyboardSleep),
        ggez::input::keyboard::KeyCode::Stop => Some(Input::KeyboardStop),
        ggez::input::keyboard::KeyCode::Sysrq => Some(Input::KeyboardSysrq),
        ggez::input::keyboard::KeyCode::Tab => Some(Input::KeyboardTab),
        ggez::input::keyboard::KeyCode::Underline => Some(Input::KeyboardUnderline),
        ggez::input::keyboard::KeyCode::Unlabeled => Some(Input::KeyboardUnlabeled),
        ggez::input::keyboard::KeyCode::VolumeDown => Some(Input::KeyboardVolumeDown),
        ggez::input::keyboard::KeyCode::VolumeUp => Some(Input::KeyboardVolumeUp),
        ggez::input::keyboard::KeyCode::Wake => Some(Input::KeyboardWake),
        ggez::input::keyboard::KeyCode::WebBack => Some(Input::KeyboardWebBack),
        ggez::input::keyboard::KeyCode::WebFavorites => Some(Input::KeyboardWebFavorites),
        ggez::input::keyboard::KeyCode::WebForward => Some(Input::KeyboardWebForward),
        ggez::input::keyboard::KeyCode::WebHome => Some(Input::KeyboardWebHome),
        ggez::input::keyboard::KeyCode::WebRefresh => Some(Input::KeyboardWebRefresh),
        ggez::input::keyboard::KeyCode::WebSearch => Some(Input::KeyboardWebSearch),
        ggez::input::keyboard::KeyCode::WebStop => Some(Input::KeyboardWebStop),
        ggez::input::keyboard::KeyCode::Yen => Some(Input::KeyboardYen),
        ggez::input::keyboard::KeyCode::Copy => Some(Input::KeyboardCopy),
        ggez::input::keyboard::KeyCode::Paste => Some(Input::KeyboardPaste),
        ggez::input::keyboard::KeyCode::Cut => Some(Input::KeyboardCut),
    }
}

pub fn pressed(ctx: &ggez::Context, input: Input) -> bool {
    if let Some(key_code) = translate_input_to_keycode(input) {
        ctx.keyboard.is_key_pressed(key_code)
    } else if let Some(mouse_button) = translate_input_to_mousebutton(input) {
        ctx.mouse.button_pressed(mouse_button)
    } else {
        error!("Input could not be translated: {input:?}");
        false
    }
}