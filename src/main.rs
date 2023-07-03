use slint::LogicalPosition;

slint::include_modules!();

fn main() {

    let main = MainWindow::new().expect("Failed to create new Main Window Component");

    let close_handle = main.as_weak();
    main.on_close_window(move ||{
        close_handle.upgrade().expect("Failed to upgrade Close Handle").hide().expect("Failed to Close Window");
    });

    let move_handle = main.as_weak();
    main.on_move_window(move |offset_x, offset_y|{
        let move_handle = move_handle.upgrade().expect("Failed to upgrade Move Handle");
        let logical_pos = move_handle.window().position().to_logical(move_handle.window().scale_factor());
        dbg!(logical_pos);
        dbg!(offset_x);
        dbg!(offset_y);
        move_handle.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });

    main.run().expect("Failed to run main Window Component Event Loop");
}
