use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use rdev::{EventType, Key};
use tao::{dpi::{LogicalPosition, LogicalSize, PhysicalPosition}, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use wry::WebViewBuilder;

const YT_URL: &str = "https://youtube.com/shorts";
const IG_URL: &str = "https://instagram.com/reels";
const N: i32 = 24;

// nominal values (assuming 1920x1080, 1.0 scale) for width/height of grid cells
const YT_CELL_W: i32 = 316;
const YT_CELL_H: i32 = 616;
const IG_CELL_W: i32 = 400;
const IG_CELL_H: i32 = 540;

const NAV_H: i32 = 80;

fn main() {
	let quit_flag = Arc::new(AtomicBool::new(false));
	let thread_quit_flag = quit_flag.clone();

	let key_event_handler = DeviceEventsHandler::new(Duration::from_millis(10)).unwrap();
	let _key_listener = key_event_handler.on_key_down(move |keycode| {
		if keycode == &Keycode::Q {
			println!("q pressed");
			thread_quit_flag.store(true, Ordering::SeqCst);
		}
	});

	let event_loop = EventLoop::new();
	let mut wvs = Vec::new();
	let mut wins = Vec::new();

	let primary_monitor = event_loop.primary_monitor().unwrap();
	let scale_factor = primary_monitor.scale_factor();

	let monitor_height = primary_monitor.size().height as i32;
	let monitor_width = primary_monitor.size().width as i32;

	let mut x = 0;
	let mut y : i32;

	let mut row = 0;
	let mut nom_w = 0;
	let mut num_screens = N;
	
	for i in 0..N {
		x = x + nom_w; // increment x by previous cell width
		// Alternate between Youtube Shorts and Instagram Reels
		let nom_h : i32;
		let url : &str;
		if i % 2 == 0 {
			nom_w = YT_CELL_W;
			nom_h = YT_CELL_H;
			url = YT_URL;
		} else {
			nom_w = IG_CELL_W;
			nom_h = IG_CELL_H;
			url = IG_URL;
		}
		
		// If the next cell would exceed the monitor width, reset x and increment row
		if x >= monitor_width {
			x = 0;
			row += 1;
		}

		// Calculate y based on height of the current cell type
		if i % 2 == 0 {
			y = row * YT_CELL_H;
		} else {
			y = row * IG_CELL_H;
		}

		// If y exceeds monitor height, stop creating more windows
		if y - nom_h > monitor_height {
			println!("Screen has been filled to maximum capacity");
			num_screens = i;
			break;
		}

		let scl_cell_w = nom_w as f64 / scale_factor;
		let scl_cell_h = nom_h as f64 / scale_factor;

		let window = WindowBuilder::new()
			.with_title(format!("Grid {i}"))
			.with_inner_size(LogicalSize::new(scl_cell_w, scl_cell_h))
			.with_position(LogicalPosition::new(x, y))
			.with_decorations(false)
			.build(&event_loop)
			.unwrap();
		
		let y_off = (row + 1) * NAV_H;
		window.set_outer_position(PhysicalPosition::new(x, y - y_off));
		if row == 0 {
			window.set_always_on_top(true);
		}

		let wv = WebViewBuilder::new()
			.with_url(url)
			.build(&window)
			.unwrap();

		wv.zoom(1.0 / scale_factor).unwrap();

		wvs.push(wv);
		wins.push(Arc::new(window));
	}

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		match event {
			Event::WindowEvent {
				window_id,
				event: WindowEvent::Resized(size),
				..
			} => {
				println!("Window {:?} resized to {} x {}", window_id, size.width, size.height);
			},

			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
				*control_flow = ControlFlow::Exit;
			},

			Event::MainEventsCleared => {
				if quit_flag.load(Ordering::SeqCst) {
					println!("quitting");
					*control_flow = ControlFlow::Exit;
				}

				if rand::random_bool(0.001) {
					let i = rand::random_range(0..num_screens) as usize;
					let w = wins[i].clone();

					thread::spawn(move || {
						println!("lucky {i}");
						w.set_focus();
						thread::sleep(Duration::from_millis(200));
						rdev::simulate(&EventType::KeyPress(Key::DownArrow)).unwrap();
						rdev::simulate(&EventType::KeyRelease(Key::DownArrow)).unwrap();
					});
				}
			},

			_ => {}
		}
	});
}
