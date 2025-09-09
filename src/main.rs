use std::{process::Command, thread::sleep, time::Duration};
use windows::core::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible, SetWindowPos, SWP_SHOWWINDOW, WNDENUMPROC};
use windows::Win32::Foundation::{HWND, LPARAM};


fn grid_all_windows() {
	// let chrome_windows: Vec<_> = x_win::get_open_windows().unwrap().into_iter()
	// 	.filter(|w| w.title.contains("Chrome"))
	// 	.collect();




	// if chrome_windows.is_empty() {
	// 	eprintln!("No chrome windows");
	// 	return;
	// }

	// for w in chrome_windows {
	// 	println!("{}", w.title);
	// }
	// let count = chrome_windows.len();
	// let cols = (count as f64).sqrt().ceil() as i32;
	// let rows = ((count as f64) / cols as f64).ceil() as i32;

	println!("r{rows}c{cols}");

	let (screen_w, screen_h) = unsafe {
		use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SYSTEM_METRICS_INDEX};
		(GetSystemMetrics(SYSTEM_METRICS_INDEX(0)), GetSystemMetrics(SYSTEM_METRICS_INDEX(1)))
	};

	println!("{screen_w}x{screen_h}");

	// let cell_w = screen_w / cols;
	// let cell_h = screen_h / rows;

	// for (i, win) in chrome_windows.into_iter().enumerate() {
	// 	let hwnd = HWND(win.id as isize);
	// 	let row = i as i32 / cols;
	// 	let col = i as i32 % cols;
	// 	let x = col * cell_w;
	// 	let y = row * cell_h;

	// 	unsafe {
	// 		SetWindowPos(hwnd, HWND(0), x, y, cell_w, cell_h, SWP_SHOWWINDOW);
	// 	}
	// }
	
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
	let buf = [0u16; 512];
	let len = GetWindowTextW(hwnd, &mut buf);

	if len > 0 && IsWindowVisible(hwnd).as_bool() {
		println!("hel");
	}
	return BOOL(1);
}

fn main() {
	const URL: &str = "https://www.youtube.com/shorts";
	let args = ["/c", "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome", "--new-window", URL];

	let all_windows: Vec<HWND> = Vec::new();
	// EnumWindows(Some(enum_windows_proc), LPARAM(&mut all_windows as *mut _ as isize))

	Command::new("cmd")
		.args(args)
		.spawn()
		.unwrap();

	sleep(Duration::from_secs(1));

	grid_all_windows();
}
