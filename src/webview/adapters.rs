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
            KeyCode::KeyC => Ok(Self(67)),
            KeyCode::KeyD => Ok(Self(68)),
            KeyCode::KeyF => Ok(Self(70)),
            KeyCode::KeyG => Ok(Self(71)),
            KeyCode::KeyH => Ok(Self(72)),
            KeyCode::KeyI => Ok(Self(73)),
            KeyCode::KeyR => Ok(Self(82)),
            KeyCode::KeyS => Ok(Self(83)),
            KeyCode::KeyV => Ok(Self(86)),
            KeyCode::KeyX => Ok(Self(88)),
            KeyCode::F11 => Ok(Self(122)),
            KeyCode::Minus => Ok(Self(173)),
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
            KeyCode::KeyC => Ok(Self(54)),
            KeyCode::KeyD => Ok(Self(40)),
            KeyCode::KeyF => Ok(Self(41)),
            KeyCode::KeyG => Ok(Self(42)),
            KeyCode::KeyH => Ok(Self(43)),
            KeyCode::KeyI => Ok(Self(31)),
            KeyCode::KeyR => Ok(Self(27)),
            KeyCode::KeyS => Ok(Self(39)),
            KeyCode::KeyV => Ok(Self(55)),
            KeyCode::KeyX => Ok(Self(53)),
            KeyCode::F11 => Ok(Self(95)),
            KeyCode::Minus => Ok(Self(20)),
            _ => Err("Failed to convert KeyCode to NativeKeyCode"),
        }
    }
}
