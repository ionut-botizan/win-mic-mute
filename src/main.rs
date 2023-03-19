#![windows_subsystem = "windows"]

use windows::Win32::{
	Foundation::{LPARAM, WPARAM},
	System::SystemServices::APPCOMMAND_MICROPHONE_VOLUME_MUTE,
	UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_APPCOMMAND},
};

fn main() {
	// @see https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand
	// @see https://stackoverflow.com/questions/40577695/why-do-wm-appcommand-lparam-have-to-be-multiplied-by-65536
	let cmd_mic_mute = (APPCOMMAND_MICROPHONE_VOLUME_MUTE.0 << 16) as isize;

	unsafe {
		let h_win = GetForegroundWindow();
		SendMessageW(h_win, WM_APPCOMMAND, WPARAM(0), LPARAM(cmd_mic_mute));
	}
}
