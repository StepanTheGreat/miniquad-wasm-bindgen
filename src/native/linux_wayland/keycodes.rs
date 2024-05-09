use crate::event::KeyCode;

pub fn translate(keysym: u32) -> KeyCode {
	// See xkbcommon/xkbcommon-keysyms.h
	match keysym {
		65307 => return KeyCode::Escape,
		65289 => return KeyCode::Tab,
		65505 => return KeyCode::LeftShift,
		65506 => return KeyCode::RightShift,
		65507 => return KeyCode::LeftControl,
		65508 => return KeyCode::RightControl,
		65511 | 65513 => return KeyCode::LeftAlt,
		65406 | 65027 | 65512 | 65514 => return KeyCode::RightAlt,
		65515 => return KeyCode::LeftSuper,
		65516 => return KeyCode::RightSuper,
		65383 => return KeyCode::Menu,
		65407 => return KeyCode::NumLock,
		65509 => return KeyCode::CapsLock,
		65377 => return KeyCode::PrintScreen,
		65300 => return KeyCode::ScrollLock,
		65299 => return KeyCode::Pause,
		65535 => return KeyCode::Delete,
		65288 => return KeyCode::Backspace,
		65293 => return KeyCode::Enter,
		65360 => return KeyCode::Home,
		65367 => return KeyCode::End,
		65365 => return KeyCode::PageUp,
		65366 => return KeyCode::PageDown,
		65379 => return KeyCode::Insert,
		65361 => return KeyCode::Left,
		65363 => return KeyCode::Right,
		65364 => return KeyCode::Down,
		65362 => return KeyCode::Up,
		65470 => return KeyCode::F1,
		65471 => return KeyCode::F2,
		65472 => return KeyCode::F3,
		65473 => return KeyCode::F4,
		65474 => return KeyCode::F5,
		65475 => return KeyCode::F6,
		65476 => return KeyCode::F7,
		65477 => return KeyCode::F8,
		65478 => return KeyCode::F9,
		65479 => return KeyCode::F10,
		65480 => return KeyCode::F11,
		65481 => return KeyCode::F12,
		65482 => return KeyCode::F13,
		65483 => return KeyCode::F14,
		65484 => return KeyCode::F15,
		65485 => return KeyCode::F16,
		65486 => return KeyCode::F17,
		65487 => return KeyCode::F18,
		65488 => return KeyCode::F19,
		65489 => return KeyCode::F20,
		65490 => return KeyCode::F21,
		65491 => return KeyCode::F22,
		65492 => return KeyCode::F23,
		65493 => return KeyCode::F24,
		65494 => return KeyCode::F25,
		65455 => return KeyCode::KpDivide,
		65450 => return KeyCode::KpMultiply,
		65453 => return KeyCode::KpSubtract,
		65451 => return KeyCode::KpAdd,
		65438 => return KeyCode::Kp0,
		65436 => return KeyCode::Kp1,
		65433 => return KeyCode::Kp2,
		65435 => return KeyCode::Kp3,
		65430 => return KeyCode::Kp4,
		65437 => return KeyCode::Kp5,
		65432 => return KeyCode::Kp6,
		65429 => return KeyCode::Kp7,
		65431 => return KeyCode::Kp8,
		65434 => return KeyCode::Kp9,
		65439 => return KeyCode::KpDecimal,
		65469 => return KeyCode::KpEqual,
		65421 => return KeyCode::KpEnter,
		65 | 97 => return KeyCode::A,
		66 | 98 => return KeyCode::B,
		67 | 99 => return KeyCode::C,
		68 | 100 => return KeyCode::D,
		69 | 101 => return KeyCode::E,
		70 | 102 => return KeyCode::F,
		71 | 103 => return KeyCode::G,
		72 | 104 => return KeyCode::H,
		73 | 105 => return KeyCode::I,
		74 | 106 => return KeyCode::J,
		75 | 107 => return KeyCode::K,
		76 | 108 => return KeyCode::L,
		77 | 109 => return KeyCode::M,
		78 | 110 => return KeyCode::N,
		79 | 111 => return KeyCode::O,
		80 | 112 => return KeyCode::P,
		81 | 113 => return KeyCode::Q,
		82 | 114 => return KeyCode::R,
		83 | 115 => return KeyCode::S,
		84 | 116 => return KeyCode::T,
		85 | 117 => return KeyCode::U,
		86 | 118 => return KeyCode::V,
		87 | 119 => return KeyCode::W,
		88 | 120 => return KeyCode::X,
		89 | 121 => return KeyCode::Y,
		90 | 122 => return KeyCode::Z,
		49 => return KeyCode::Key1,
		50 => return KeyCode::Key2,
		51 => return KeyCode::Key3,
		52 => return KeyCode::Key4,
		53 => return KeyCode::Key5,
		54 => return KeyCode::Key6,
		55 => return KeyCode::Key7,
		56 => return KeyCode::Key8,
		57 => return KeyCode::Key9,
		48 => return KeyCode::Key0,
		32 => return KeyCode::Space,
		45 => return KeyCode::Minus,
		61 => return KeyCode::Equal,
		91 => return KeyCode::LeftBracket,
		93 => return KeyCode::RightBracket,
		92 => return KeyCode::Backslash,
		59 => return KeyCode::Semicolon,
		39 => return KeyCode::Apostrophe,
		96 => return KeyCode::GraveAccent,
		44 => return KeyCode::Comma,
		46 => return KeyCode::Period,
		47 => return KeyCode::Slash,
		60 => return KeyCode::World1,
		_ => return KeyCode::Unknown,
	};
}
