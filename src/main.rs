use std::fs::File;

fn main() {
    let aim_file1 = File::open("draw_test.txt").unwrap();
    let mut interpreter_obj = interpreter::Interpreter::new(aim_file1);
    interpreter_obj.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);
    let point_result = interpreter_obj.interpret().unwrap();

    let mut drawer_obj = drawer::Drawer::new()
        .build_image_size(1280, 720)
        .build_coordinate_range(-10.0, 20.0, -10.0, 20.0)
        .build_message("draw_test.png", "");

    drawer_obj.add_task(point_result, drawer::colors::RED);
    drawer_obj.draw();
}



