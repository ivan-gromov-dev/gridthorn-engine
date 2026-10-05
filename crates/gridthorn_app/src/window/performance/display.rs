use winit::window::Window;

pub(super) fn report(window: &Window) {
    let size = window.inner_size();
    let monitor = window.current_monitor();
    let name = monitor
        .as_ref()
        .and_then(winit::monitor::MonitorHandle::name);
    let extent = monitor.as_ref().map(|monitor| {
        let size = monitor.size();
        [size.width, size.height]
    });
    let origin = monitor.as_ref().map(|monitor| {
        let position = monitor.position();
        [position.x, position.y]
    });
    let refresh = monitor
        .as_ref()
        .and_then(winit::monitor::MonitorHandle::refresh_rate_millihertz)
        .filter(|rate| *rate != 0);
    eprintln!(
        "window_configuration,physical_width={},physical_height={},scale_factor={},monitor_name={name:?},monitor_extent={extent:?},monitor_origin={origin:?},refresh_millihertz={refresh:?}",
        size.width,
        size.height,
        window.scale_factor(),
    );
}
