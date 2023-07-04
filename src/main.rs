slint::include_modules!();

fn main() {

    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new())).unwrap();
    let main = MainWindow::new().expect("Failed to create new Main Window Component");

    let close_handle = main.as_weak();
    main.on_close_window(move ||{
        close_handle.upgrade().expect("Failed to upgrade Close Handle").hide().expect("Failed to Close Window");
    });

    let move_handle = main.as_weak();
    main.on_move_window(move ||{
        let move_handle = move_handle.upgrade().expect("Failed to upgrade Move Handle");
        i_slint_backend_winit::WinitWindowAccessor::with_winit_window(move_handle.window(), |win| win.drag_window());
    });

    main.run().expect("Failed to run main Window Component Event Loop");
}
