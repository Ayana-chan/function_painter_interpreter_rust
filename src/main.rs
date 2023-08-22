use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //指定输入
    let aim_file = File::open("draw_test.txt").unwrap();
    let mut interpreter_obj = interpreter::Interpreter::new(aim_file);
    //限制坐标范围
    interpreter_obj.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);
    let point_result = interpreter_obj.interpret().unwrap();

    let mut drawer_obj = drawer::Drawer::new()
        //指定输出图像大小
        .build_image_size(720, 720)
        //指定坐标轴显示范围
        .build_coordinate_range(-10.0, 20.0, -10.0, 20.0)
        //指定输出文件名和标题
        .build_message("draw_test.png", "");

    //添加点集和颜色
    drawer_obj.add_task(point_result, drawer::colors::RED);
    drawer_obj.draw()
}

// fn main() -> Result<(), Box<dyn std::error::Error>>{
//     //指定输入1
//     let aim_file1 = File::open("test_file1.txt").unwrap();
//     let mut interpreter_obj1 = interpreter::Interpreter::new(aim_file1);
//     //限制坐标范围
//     interpreter_obj1.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);
//     let point_result1 = interpreter_obj1.interpret().unwrap();
//
//     //指定输入2
//     let aim_file2 = File::open("test_file2.txt").unwrap();
//     let mut interpreter_obj2 = interpreter::Interpreter::new(aim_file2);
//     //限制坐标范围
//     interpreter_obj2.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);
//     let point_result2 = interpreter_obj2.interpret().unwrap();
//
//     let mut drawer_obj = drawer::Drawer::new()
//         //指定输出图像大小
//         .build_image_size(1280, 720)
//         //指定坐标轴显示范围
//         .build_coordinate_range(-10.0, 20.0, -10.0, 20.0)
//         //指定输出文件名和标题
//         .build_message("draw_test.png", "First Test");
//
//     //添加点集和颜色
//     drawer_obj.add_task(point_result1, drawer::colors::RED);
//     drawer_obj.add_task(point_result2, drawer::colors::BLUE);
//     drawer_obj.draw()
// }



