use std::fs::File;

fn main() {
    let aim_file1 = File::open("draw_test.txt").unwrap();
    let point_result = interpreter::interpret(aim_file1).unwrap();
    let drawer_obj = drawer::Drawer::new()
        .build_image_size(1280,720)
        .build_coordinate_range(-10.0,20.0,-10.0,20.0)
        .build_message("draw_test","")
        .draw(point_result,drawer::colors::RED);
}



