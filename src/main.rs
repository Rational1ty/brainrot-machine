use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use rdev::{EventType, Key};
use tao::{dpi::{LogicalPosition, LogicalSize, PhysicalPosition}, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use wry::WebViewBuilder;

const URL: &str = "https://youtube.com/shorts";
const N: i32 = 12;
const COLS: i32 = 6;

// nominal values (assuming 1920x1080, 1.0 scale) for width/height of grid cells
const NOM_W: i32 = 316;
const NOM_H: i32 = 616;

const YT_NAV_H: i32 = 80;

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

	let scl_cell_w = NOM_W as f64 / scale_factor;
	let scl_cell_h = NOM_H as f64 / scale_factor;

	for i in 0..N {
		let col = i % COLS;
		let row = i / COLS;
		let x = (col * NOM_W) as i32;
		let y = (row * NOM_H) as i32;

		let window = WindowBuilder::new()
			.with_title(format!("Grid {i}"))
			.with_inner_size(LogicalSize::new(scl_cell_w, scl_cell_h))
			.with_position(LogicalPosition::new(x, y))
			.with_decorations(false)
			.build(&event_loop)
			.unwrap();
		
		let y_off = (row + 1) * YT_NAV_H;
		window.set_outer_position(PhysicalPosition::new(x, y - y_off));
		if row == 0 {
			window.set_always_on_top(true);
		}

		let wv = WebViewBuilder::new()
			.with_url(URL)
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
					let i = rand::random_range(0..N) as usize;
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
