use tao::{dpi::{LogicalPosition, LogicalSize, PhysicalPosition}, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, platform::windows::WindowBuilderExtWindows, window::WindowBuilder};
use wry::WebViewBuilder;


fn main() {
	let event_loop = EventLoop::new();
	let mut wvs = Vec::new();
	let mut wins = Vec::new();

	let primary = event_loop.primary_monitor().unwrap();
	let ssize = primary.scale_factor();
	eprintln!("ssize = {:?}", ssize);

	let url = "https://youtube.com/shorts";
	let n = 3;
	let cols = 6;
	let cell_w = 316;
	let cell_h = 616;
	let scl_cell_w = 210;
	let scl_cell_h = 410;
	let yt_nav_h = 80;

	for i in 0..n {
		let col = i % cols;
		let row = i / cols;
		let x = (col * cell_w) as i32;
		let y = (row * cell_h) as i32;

		let window = WindowBuilder::new()
			.with_title(format!("Grid {i}"))
			.with_inner_size(LogicalSize::new(scl_cell_w, scl_cell_h))
			.with_position(LogicalPosition::new(x, y))
			.with_decorations(false)
			.build(&event_loop)
			.unwrap();
		
		let y_off = (row + 1) * yt_nav_h;
		window.set_outer_position(PhysicalPosition::new(x, y - y_off));
		if row == 0 {
			window.set_always_on_top(true);
		}

		let wv = WebViewBuilder::new()
			.with_url(url)
			.build(&window)
			.unwrap();

		wv.zoom(0.67).unwrap();

		wvs.push(wv);
		wins.push(window);
	}

	// Run until the user closes all windows
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::WindowEvent {
				window_id,
				event: WindowEvent::Resized(size),
				..
			} => {
				println!(
					"Window {:?} resized to {} x {}",
					window_id,
					size.width,
					size.height
				);
			}
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				*control_flow = ControlFlow::Exit;
			}
			_ => {}
		}
	});
}

