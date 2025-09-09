use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::{process::Command, thread::sleep, time::Duration};
use windows::core::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetSystemMetrics, GetWindowTextW, IsWindowVisible, SetWindowPos, ShowWindow, SM_CXSIZEFRAME, SM_CYCAPTION, SM_CYSIZEFRAME, SWP_SHOWWINDOW, SW_RESTORE, SYSTEM_METRICS_INDEX};
use windows::Win32::Foundation::{HWND, LPARAM};


unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
	let mut buf = [0u16; 512];

	unsafe {
		let len = GetWindowTextW(hwnd, &mut buf);

		if len > 0 && IsWindowVisible(hwnd).as_bool() {
			let len_idx = len as usize;
			let title = OsString::from_wide(&buf[..len_idx]).to_string_lossy().to_string();

			if title.contains("Chrome") {
				let window_list = lparam.0 as *mut Vec<HWND>;
				(*window_list).push(hwnd);
			}
		}
	}

	// continue enumeration
	return BOOL(1);
}

fn grid_windows() {
	let mut chrome_windows: Vec<HWND> = Vec::new();
	unsafe {
		EnumWindows(Some(enum_windows_proc), LPARAM(&mut chrome_windows as *mut _ as isize)).unwrap();
	}

	let n = chrome_windows.len();
	if n == 0 {
		eprintln!("No open chrome windows");
		return;
	}

	let cols = (n as f64).sqrt().ceil() as i32;
	let rows = ((n as f64) / cols as f64).ceil() as i32;

	let (screen_w, screen_h) = unsafe {
		(GetSystemMetrics(SYSTEM_METRICS_INDEX(0)), GetSystemMetrics(SYSTEM_METRICS_INDEX(1)))
	};
	println!("{screen_w}x{screen_h}");

	// extra space around windows
	let frame_x = unsafe { GetSystemMetrics(SM_CXSIZEFRAME) };
	let frame_y = unsafe { GetSystemMetrics(SM_CYSIZEFRAME) };
	let caption  = unsafe { GetSystemMetrics(SM_CYCAPTION) };

	let border_x = 2 * frame_x;
	let border_y = 2 * frame_y + caption;

	let w = screen_w / cols - border_x;
	let h = screen_h / rows - border_y;

	for (i, hwnd) in chrome_windows.iter().enumerate() {
		let r = i as i32 / cols;
		let c = i as i32 % cols;
		let x = c * w;
		let y = r * h;
		unsafe {
			ShowWindow(*hwnd, SW_RESTORE).unwrap();
			SetWindowPos(*hwnd, None, x, y, w, h, SWP_SHOWWINDOW).unwrap();
		}
	}
}

fn main() {
	const URL: &str = "https://www.youtube.com/shorts";
	let args = ["/c", "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome", "--new-window", URL];

	for _ in 0..4 {
		Command::new("cmd")
			.args(args)
			.spawn()
			.unwrap();
		println!("spawned window");
		sleep(Duration::from_millis(500));
	}

	grid_windows();
}
