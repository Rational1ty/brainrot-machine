use tao::{dpi::{LogicalPosition, LogicalSize}, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use wry::WebViewBuilder;

fn main() {
	let event_loop = EventLoop::new();
	let mut wvs = Vec::new();
	let mut wins = Vec::new();

	let url = "https://youtube.com/shorts";
	let n = 1;
	let cols = 4;
	let cell_w = 640;
	let cell_h = 360;

	for i in 0..n {
		let col = i % cols;
		let row = i / cols;
		let x = (col * cell_w) as i32;
		let y = (row * cell_h) as i32;

		let window = WindowBuilder::new()
			.with_title(format!("Grid {i}"))
			.with_inner_size(LogicalSize::new(cell_w, cell_h))
			.with_position(LogicalPosition::new(x, y))
			.with_decorations(false)  // seamless
			.build(&event_loop)
			.unwrap();

		let wv = WebViewBuilder::new()
			.with_url(url)
			.build(&window)
			.unwrap();

		wvs.push(wv);
		wins.push(window);
	}

	// Run until the user closes all windows
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
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

