use cef_dll_sys::cef_cursor_type_t;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::shared::types::{Cursor, MouseState};

impl From<MouseState> for cef::MouseEvent {
    fn from(state: MouseState) -> Self {
        let modifiers = match state.button {
            MouseButton::Left if state.pressed => 16,
            MouseButton::Right if state.pressed => 32,
            MouseButton::Middle if state.pressed => 64,
            _ => 0,
        };

        Self {
            x: state.position.0,
            y: state.position.1,
            modifiers,
        }
    }
}

impl From<cef::CursorType> for Cursor {
    fn from(value: cef::CursorType) -> Self {
        match value.as_ref() {
            cef_cursor_type_t::CT_POINTER => Cursor::Default,
            cef_cursor_type_t::CT_HAND => Cursor::Pointer,
            cef_cursor_type_t::CT_IBEAM => Cursor::Text,
            cef_cursor_type_t::CT_MOVE => Cursor::Move,
            cef_cursor_type_t::CT_ZOOMIN => Cursor::ZoomIn,
            cef_cursor_type_t::CT_ZOOMOUT => Cursor::ZoomOut,
            cef_cursor_type_t::CT_WAIT => Cursor::Wait,
            cef_cursor_type_t::CT_NONE => Cursor::None,
            _ => Cursor::Default,
        }
    }
}

pub struct WindowsKeyCode(pub i32);

impl TryFrom<KeyCode> for WindowsKeyCode {
    type Error = &'static str;

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Backspace => Ok(Self(8)),
            KeyCode::Tab => Ok(Self(9)),
            KeyCode::Enter => Ok(Self(13)),
            KeyCode::Escape => Ok(Self(27)),
            KeyCode::Space => Ok(Self(32)),
            KeyCode::PageUp => Ok(Self(33)),
            KeyCode::PageDown => Ok(Self(34)),
            KeyCode::End => Ok(Self(35)),
            KeyCode::Home => Ok(Self(36)),
            KeyCode::ArrowLeft => Ok(Self(37)),
            KeyCode::ArrowUp => Ok(Self(38)),
            KeyCode::ArrowRight => Ok(Self(39)),
            KeyCode::ArrowDown => Ok(Self(40)),
            KeyCode::Digit0 => Ok(Self(48)),
            KeyCode::Digit1 => Ok(Self(49)),
            KeyCode::Digit2 => Ok(Self(50)),
            KeyCode::Digit3 => Ok(Self(51)),
            KeyCode::Digit4 => Ok(Self(52)),
            KeyCode::Digit5 => Ok(Self(53)),
            KeyCode::Digit6 => Ok(Self(54)),
            KeyCode::Digit7 => Ok(Self(55)),
            KeyCode::Digit8 => Ok(Self(56)),
            KeyCode::Digit9 => Ok(Self(57)),
            KeyCode::Equal => Ok(Self(61)),
            KeyCode::KeyA => Ok(Self(65)),
            KeyCode::KeyB => Ok(Self(66)),
            KeyCode::KeyC => Ok(Self(67)),
            KeyCode::KeyD => Ok(Self(68)),
            KeyCode::KeyE => Ok(Self(69)),
            KeyCode::KeyF => Ok(Self(70)),
            KeyCode::KeyG => Ok(Self(71)),
            KeyCode::KeyH => Ok(Self(72)),
            KeyCode::KeyI => Ok(Self(73)),
            KeyCode::KeyJ => Ok(Self(74)),
            KeyCode::KeyK => Ok(Self(75)),
            KeyCode::KeyL => Ok(Self(76)),
            KeyCode::KeyM => Ok(Self(77)),
            KeyCode::KeyN => Ok(Self(78)),
            KeyCode::KeyO => Ok(Self(79)),
            KeyCode::KeyP => Ok(Self(80)),
            KeyCode::KeyQ => Ok(Self(81)),
            KeyCode::KeyR => Ok(Self(82)),
            KeyCode::KeyS => Ok(Self(83)),
            KeyCode::KeyT => Ok(Self(84)),
            KeyCode::KeyU => Ok(Self(85)),
            KeyCode::KeyV => Ok(Self(86)),
            KeyCode::KeyW => Ok(Self(87)),
            KeyCode::KeyX => Ok(Self(88)),
            KeyCode::KeyY => Ok(Self(89)),
            KeyCode::KeyZ => Ok(Self(90)),
            KeyCode::F1 => Ok(Self(112)),
            KeyCode::F2 => Ok(Self(113)),
            KeyCode::F3 => Ok(Self(114)),
            KeyCode::F4 => Ok(Self(115)),
            KeyCode::F5 => Ok(Self(116)),
            KeyCode::F6 => Ok(Self(117)),
            KeyCode::F7 => Ok(Self(118)),
            KeyCode::F8 => Ok(Self(119)),
            KeyCode::F9 => Ok(Self(120)),
            KeyCode::F10 => Ok(Self(121)),
            KeyCode::F11 => Ok(Self(122)),
            KeyCode::F12 => Ok(Self(123)),
            KeyCode::Insert => Ok(Self(45)),
            KeyCode::Delete => Ok(Self(46)),
            KeyCode::Minus => Ok(Self(173)),
            KeyCode::Comma => Ok(Self(188)),
            KeyCode::Period => Ok(Self(190)),
            KeyCode::Slash => Ok(Self(191)),
            KeyCode::Semicolon => Ok(Self(186)),
            KeyCode::Quote => Ok(Self(222)),
            KeyCode::BracketLeft => Ok(Self(219)),
            KeyCode::BracketRight => Ok(Self(221)),
            KeyCode::Backslash => Ok(Self(220)),
            KeyCode::Backquote => Ok(Self(192)),
            _ => Err("Failed to convert KeyCode to WindowsKeyCode"),
        }
    }
}

pub struct NativeKeyCode(pub i32);

impl TryFrom<KeyCode> for NativeKeyCode {
    type Error = &'static str;

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Backspace => Ok(Self(22)),
            KeyCode::Tab => Ok(Self(23)),
            KeyCode::Enter => Ok(Self(36)),
            KeyCode::Escape => Ok(Self(9)),
            KeyCode::Space => Ok(Self(65)),
            KeyCode::PageUp => Ok(Self(112)),
            KeyCode::PageDown => Ok(Self(117)),
            KeyCode::End => Ok(Self(115)),
            KeyCode::Home => Ok(Self(110)),
            KeyCode::ArrowLeft => Ok(Self(113)),
            KeyCode::ArrowUp => Ok(Self(111)),
            KeyCode::ArrowRight => Ok(Self(114)),
            KeyCode::ArrowDown => Ok(Self(116)),
            KeyCode::Digit0 => Ok(Self(19)),
            KeyCode::Digit1 => Ok(Self(10)),
            KeyCode::Digit2 => Ok(Self(11)),
            KeyCode::Digit3 => Ok(Self(12)),
            KeyCode::Digit4 => Ok(Self(13)),
            KeyCode::Digit5 => Ok(Self(14)),
            KeyCode::Digit6 => Ok(Self(15)),
            KeyCode::Digit7 => Ok(Self(16)),
            KeyCode::Digit8 => Ok(Self(17)),
            KeyCode::Digit9 => Ok(Self(18)),
            KeyCode::Equal => Ok(Self(21)),
            KeyCode::KeyA => Ok(Self(38)),
            KeyCode::KeyB => Ok(Self(56)),
            KeyCode::KeyC => Ok(Self(54)),
            KeyCode::KeyD => Ok(Self(40)),
            KeyCode::KeyE => Ok(Self(26)),
            KeyCode::KeyF => Ok(Self(41)),
            KeyCode::KeyG => Ok(Self(42)),
            KeyCode::KeyH => Ok(Self(43)),
            KeyCode::KeyI => Ok(Self(31)),
            KeyCode::KeyJ => Ok(Self(44)),
            KeyCode::KeyK => Ok(Self(45)),
            KeyCode::KeyL => Ok(Self(46)),
            KeyCode::KeyM => Ok(Self(58)),
            KeyCode::KeyN => Ok(Self(57)),
            KeyCode::KeyO => Ok(Self(32)),
            KeyCode::KeyP => Ok(Self(33)),
            KeyCode::KeyQ => Ok(Self(24)),
            KeyCode::KeyR => Ok(Self(27)),
            KeyCode::KeyS => Ok(Self(39)),
            KeyCode::KeyT => Ok(Self(28)),
            KeyCode::KeyU => Ok(Self(30)),
            KeyCode::KeyV => Ok(Self(55)),
            KeyCode::KeyW => Ok(Self(25)),
            KeyCode::KeyX => Ok(Self(53)),
            KeyCode::KeyY => Ok(Self(29)),
            KeyCode::KeyZ => Ok(Self(52)),
            KeyCode::F1 => Ok(Self(67)),
            KeyCode::F2 => Ok(Self(68)),
            KeyCode::F3 => Ok(Self(69)),
            KeyCode::F4 => Ok(Self(70)),
            KeyCode::F5 => Ok(Self(71)),
            KeyCode::F6 => Ok(Self(72)),
            KeyCode::F7 => Ok(Self(73)),
            KeyCode::F8 => Ok(Self(74)),
            KeyCode::F9 => Ok(Self(75)),
            KeyCode::F10 => Ok(Self(76)),
            KeyCode::F11 => Ok(Self(95)),
            KeyCode::F12 => Ok(Self(96)),
            KeyCode::Insert => Ok(Self(118)),
            KeyCode::Delete => Ok(Self(119)),
            KeyCode::Minus => Ok(Self(20)),
            KeyCode::Comma => Ok(Self(59)),
            KeyCode::Period => Ok(Self(60)),
            KeyCode::Slash => Ok(Self(61)),
            KeyCode::Semicolon => Ok(Self(47)),
            KeyCode::Quote => Ok(Self(48)),
            KeyCode::BracketLeft => Ok(Self(34)),
            KeyCode::BracketRight => Ok(Self(35)),
            KeyCode::Backslash => Ok(Self(51)),
            KeyCode::Backquote => Ok(Self(49)),
            _ => Err("Failed to convert KeyCode to NativeKeyCode"),
        }
    }
}
