use crate::rdev::Key;

const ALT: u32 = 64;
const ALT_GR: u32 = 108;
const BACKSPACE: u32 = 22;
const CAPS_LOCK: u32 = 66;
const CONTROL_LEFT: u32 = 37;
const CONTROL_RIGHT: u32 = 105;
const DELETE: u32 = 119;
const DOWN_ARROW: u32 = 116;
const END: u32 = 115;
const ESCAPE: u32 = 9;
const F1: u32 = 67;
const F10: u32 = 76;
const F11: u32 = 95;
const F12: u32 = 96;
const F2: u32 = 68;
const F3: u32 = 69;
const F4: u32 = 70;
const F5: u32 = 71;
const F6: u32 = 72;
const F7: u32 = 73;
const F8: u32 = 74;
const F9: u32 = 75;
const HOME: u32 = 110;
const LEFT_ARROW: u32 = 113;
const META: u32 = 133;
const PAGE_DOWN: u32 = 117;
const PAGE_UP: u32 = 112;
const RETURN: u32 = 36;
const RIGHT_ARROW: u32 = 114;
const SHIFT_LEFT: u32 = 50;
const SHIFT_RIGHT: u32 = 62;
const SPACE: u32 = 65;
const TAB: u32 = 23;
const UP_ARROW: u32 = 111;
const PRINT_SCREEN: u32 = 107;
const SCROLL_LOCK: u32 = 78;
const PAUSE: u32 = 127;
const NUM_LOCK: u32 = 77;
const BACK_QUOTE: u32 = 49;
const NUM1: u32 = 10;
const NUM2: u32 = 11;
const NUM3: u32 = 12;
const NUM4: u32 = 13;
const NUM5: u32 = 14;
const NUM6: u32 = 15;
const NUM7: u32 = 16;
const NUM8: u32 = 17;
const NUM9: u32 = 18;
const NUM0: u32 = 19;
const MINUS: u32 = 20;
const EQUAL: u32 = 21;
const KEY_Q: u32 = 24;
const KEY_W: u32 = 25;
const KEY_E: u32 = 26;
const KEY_R: u32 = 27;
const KEY_T: u32 = 28;
const KEY_Y: u32 = 29;
const KEY_U: u32 = 30;
const KEY_I: u32 = 31;
const KEY_O: u32 = 32;
const KEY_P: u32 = 33;
const LEFT_BRACKET: u32 = 34;
const RIGHT_BRACKET: u32 = 35;
const KEY_A: u32 = 38;
const KEY_S: u32 = 39;
const KEY_D: u32 = 40;
const KEY_F: u32 = 41;
const KEY_G: u32 = 42;
const KEY_H: u32 = 43;
const KEY_J: u32 = 44;
const KEY_K: u32 = 45;
const KEY_L: u32 = 46;
const SEMI_COLON: u32 = 47;
const QUOTE: u32 = 48;
const BACK_SLASH: u32 = 51;
const INTL_BACKSLASH: u32 = 94;
const KEY_Z: u32 = 52;
const KEY_X: u32 = 53;
const KEY_C: u32 = 54;
const KEY_V: u32 = 55;
const KEY_B: u32 = 56;
const KEY_N: u32 = 57;
const KEY_M: u32 = 58;
const COMMA: u32 = 59;
const DOT: u32 = 60;
const SLASH: u32 = 61;
const INSERT: u32 = 118;
const KP_RETURN: u32 = 104;
const KP_MINUS: u32 = 82;
const KP_PLUS: u32 = 86;
const KP_MULTIPLY: u32 = 63;
const KP_DIVIDE: u32 = 106;
const KP0: u32 = 90;
const KP1: u32 = 87;
const KP2: u32 = 88;
const KP3: u32 = 89;
const KP4: u32 = 83;
const KP5: u32 = 84;
const KP6: u32 = 85;
const KP7: u32 = 79;
const KP8: u32 = 80;
const KP9: u32 = 81;
const KP_DELETE: u32 = 91;

pub fn code_from_key(key: &Key) -> Option<u16> {
    match key {
        Key::Alt => Some(ALT),
        Key::AltGr => Some(ALT_GR),
        Key::Backspace => Some(BACKSPACE),
        Key::CapsLock => Some(CAPS_LOCK),
        Key::ControlLeft => Some(CONTROL_LEFT),
        Key::ControlRight => Some(CONTROL_RIGHT),
        Key::Delete => Some(DELETE),
        Key::DownArrow => Some(DOWN_ARROW),
        Key::End => Some(END),
        Key::Escape => Some(ESCAPE),
        Key::F1 => Some(F1),
        Key::F10 => Some(F10),
        Key::F11 => Some(F11),
        Key::F12 => Some(F12),
        Key::F2 => Some(F2),
        Key::F3 => Some(F3),
        Key::F4 => Some(F4),
        Key::F5 => Some(F5),
        Key::F6 => Some(F6),
        Key::F7 => Some(F7),
        Key::F8 => Some(F8),
        Key::F9 => Some(F9),
        Key::Home => Some(HOME),
        Key::LeftArrow => Some(LEFT_ARROW),
        Key::MetaLeft => Some(META_LEFT),
        Key::PageDown => Some(PAGE_DOWN),
        Key::PageUp => Some(PAGE_UP),
        Key::Return => Some(RETURN),
        Key::RightArrow => Some(RIGHT_ARROW),
        Key::ShiftLeft => Some(SHIFT_LEFT),
        Key::ShiftRight => Some(SHIFT_RIGHT),
        Key::Space => Some(SPACE),
        Key::Tab => Some(TAB),
        Key::UpArrow => Some(UP_ARROW),
        Key::PrintScreen => Some(PRINT_SCREEN),
        Key::ScrollLock => Some(SCROLL_LOCK),
        Key::Pause => Some(PAUSE),
        Key::NumLock => Some(NUM_LOCK),
        Key::BackQuote => Some(BACK_QUOTE),
        Key::Num1 => Some(NUM1),
        Key::Num2 => Some(NUM2),
        Key::Num3 => Some(NUM3),
        Key::Num4 => Some(NUM4),
        Key::Num5 => Some(NUM5),
        Key::Num6 => Some(NUM6),
        Key::Num7 => Some(NUM7),
        Key::Num8 => Some(NUM8),
        Key::Num9 => Some(NUM9),
        Key::Num0 => Some(NUM0),
        Key::Minus => Some(MINUS),
        Key::Equal => Some(EQUAL),
        Key::KeyQ => Some(KEY_Q),
        Key::KeyW => Some(KEY_W),
        Key::KeyE => Some(KEY_E),
        Key::KeyR => Some(KEY_R),
        Key::KeyT => Some(KEY_T),
        Key::KeyY => Some(KEY_Y),
        Key::KeyU => Some(KEY_U),
        Key::KeyI => Some(KEY_I),
        Key::KeyO => Some(KEY_O),
        Key::KeyP => Some(KEY_P),
        Key::LeftBracket => Some(LEFT_BRACKET),
        Key::RightBracket => Some(RIGHT_BRACKET),
        Key::KeyA => Some(KEY_A),
        Key::KeyS => Some(KEY_S),
        Key::KeyD => Some(KEY_D),
        Key::KeyF => Some(KEY_F),
        Key::KeyG => Some(KEY_G),
        Key::KeyH => Some(KEY_H),
        Key::KeyJ => Some(KEY_J),
        Key::KeyK => Some(KEY_K),
        Key::KeyL => Some(KEY_L),
        Key::SemiColon => Some(SEMI_COLON),
        Key::Quote => Some(QUOTE),
        Key::BackSlash => Some(BACK_SLASH),
        Key::IntlBackslash => Some(INTL_BACKSLASH),
        Key::KeyZ => Some(KEY_Z),
        Key::KeyX => Some(KEY_X),
        Key::KeyC => Some(KEY_C),
        Key::KeyV => Some(KEY_V),
        Key::KeyB => Some(KEY_B),
        Key::KeyN => Some(KEY_N),
        Key::KeyM => Some(KEY_M),
        Key::Comma => Some(COMMA),
        Key::Dot => Some(DOT),
        Key::Slash => Some(SLASH),
        Key::Insert => Some(INSERT),
        Key::KpReturn => Some(KP_RETURN),
        Key::KpMinus => Some(KP_MINUS),
        Key::KpPlus => Some(KP_PLUS),
        Key::KpMultiply => Some(KP_MULTIPLY),
        Key::KpDivide => Some(KP_DIVIDE),
        Key::Kp0 => Some(KP0),
        Key::Kp1 => Some(KP1),
        Key::Kp2 => Some(KP2),
        Key::Kp3 => Some(KP3),
        Key::Kp4 => Some(KP4),
        Key::Kp5 => Some(KP5),
        Key::Kp6 => Some(KP6),
        Key::Kp7 => Some(KP7),
        Key::Kp8 => Some(KP8),
        Key::Kp9 => Some(KP9),
        Key::KpDelete => Some(KP_DELETE),
        Key::Unknown(code) => Some(*code as u16),
        _ => None,
    }
}

pub fn key_from_code(code: u32) -> Key {
    match code {
        ALT => Key::Alt,
        ALT_GR => Key::AltGr,
        BACKSPACE => Key::Backspace,
        CAPS_LOCK => Key::CapsLock,
        CONTROL_LEFT => Key::ControlLeft,
        CONTROL_RIGHT => Key::ControlRight,
        DELETE => Key::Delete,
        DOWN_ARROW => Key::DownArrow,
        END => Key::End,
        ESCAPE => Key::Escape,
        F1 => Key::F1,
        F10 => Key::F10,
        F11 => Key::F11,
        F12 => Key::F12,
        F2 => Key::F2,
        F3 => Key::F3,
        F4 => Key::F4,
        F5 => Key::F5,
        F6 => Key::F6,
        F7 => Key::F7,
        F8 => Key::F8,
        F9 => Key::F9,
        HOME => Key::Home,
        LEFT_ARROW => Key::LeftArrow,
        META_LEFT => Key::MetaLeft,
        PAGE_DOWN => Key::PageDown,
        PAGE_UP => Key::PageUp,
        RETURN => Key::Return,
        RIGHT_ARROW => Key::RightArrow,
        SHIFT_LEFT => Key::ShiftLeft,
        SHIFT_RIGHT => Key::ShiftRight,
        SPACE => Key::Space,
        TAB => Key::Tab,
        UP_ARROW => Key::UpArrow,
        PRINT_SCREEN => Key::PrintScreen,
        SCROLL_LOCK => Key::ScrollLock,
        PAUSE => Key::Pause,
        NUM_LOCK => Key::NumLock,
        BACK_QUOTE => Key::BackQuote,
        NUM1 => Key::Num1,
        NUM2 => Key::Num2,
        NUM3 => Key::Num3,
        NUM4 => Key::Num4,
        NUM5 => Key::Num5,
        NUM6 => Key::Num6,
        NUM7 => Key::Num7,
        NUM8 => Key::Num8,
        NUM9 => Key::Num9,
        NUM0 => Key::Num0,
        MINUS => Key::Minus,
        EQUAL => Key::Equal,
        KEY_Q => Key::KeyQ,
        KEY_W => Key::KeyW,
        KEY_E => Key::KeyE,
        KEY_R => Key::KeyR,
        KEY_T => Key::KeyT,
        KEY_Y => Key::KeyY,
        KEY_U => Key::KeyU,
        KEY_I => Key::KeyI,
        KEY_O => Key::KeyO,
        KEY_P => Key::KeyP,
        LEFT_BRACKET => Key::LeftBracket,
        RIGHT_BRACKET => Key::RightBracket,
        KEY_A => Key::KeyA,
        KEY_S => Key::KeyS,
        KEY_D => Key::KeyD,
        KEY_F => Key::KeyF,
        KEY_G => Key::KeyG,
        KEY_H => Key::KeyH,
        KEY_J => Key::KeyJ,
        KEY_K => Key::KeyK,
        KEY_L => Key::KeyL,
        SEMI_COLON => Key::SemiColon,
        QUOTE => Key::Quote,
        BACK_SLASH => Key::BackSlash,
        INTL_BACKSLASH => Key::IntlBackslash,
        KEY_Z => Key::KeyZ,
        KEY_X => Key::KeyX,
        KEY_C => Key::KeyC,
        KEY_V => Key::KeyV,
        KEY_B => Key::KeyB,
        KEY_N => Key::KeyN,
        KEY_M => Key::KeyM,
        COMMA => Key::Comma,
        DOT => Key::Dot,
        SLASH => Key::Slash,
        INSERT => Key::Insert,
        KP_RETURN => Key::KpReturn,
        KP_MINUS => Key::KpMinus,
        KP_PLUS => Key::KpPlus,
        KP_MULTIPLY => Key::KpMultiply,
        KP_DIVIDE => Key::KpDivide,
        KP0 => Key::Kp0,
        KP1 => Key::Kp1,
        KP2 => Key::Kp2,
        KP3 => Key::Kp3,
        KP4 => Key::Kp4,
        KP5 => Key::Kp5,
        KP6 => Key::Kp6,
        KP7 => Key::Kp7,
        KP8 => Key::Kp8,
        KP9 => Key::Kp9,
        KP_DELETE => Key::KpDelete,
        code => Key::Unknown(code),
    }
}

#[cfg(test)]
mod test {
    use super::{code_from_key, key_from_code};
    #[test]
    fn test_reversible() {
        for code in 0..65636 {
            let key = key_from_code(code);
            let code2 = code_from_key(&key);
            assert_eq!(code, code2)
        }
    }
}
