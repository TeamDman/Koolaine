use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();

    println!("Listing monitors:");
    let monitor_list = event_loop.available_monitors();
    for (ordinal, monitor) in monitor_list.enumerate() {
        let name = monitor.name().unwrap_or_else(|| "Unknown".to_string());
        let size = monitor.size();
        let position = monitor.position();
        println!(
            "Monitor #{}: {}, Position: {:?}, Size: {:?}",
            ordinal, name, position, size
        );
    }
}
