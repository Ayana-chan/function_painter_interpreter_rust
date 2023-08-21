use plotters::prelude::*;

pub use plotters::style::colors;

pub struct Drawer {
    //结果集
    point_vec_list: Vec<(Vec<(f64, f64)>, RGBColor)>,

    defined_coordinate_range_flag: bool,
    //坐标范围
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,

    //图片大小
    width: u32,
    height: u32,

    //其他信息
    file_name: String,
    caption: Option<String>,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            point_vec_list: Vec::new(),

            defined_coordinate_range_flag: false,
            //下面这四个数字实际上没啥用
            min_x: -8000.0,
            max_x: 8000.0,
            min_y: -5000.0,
            max_y: 5000.0,

            width: 640,
            height: 480,

            file_name: String::from("plot.png"),
            caption: None,
        }
    }

    ///确定坐标范围
    pub fn build_coordinate_range(mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        if self.min_x >= self.max_x {
            panic!("Drawer: min_x should be smaller than max_x.")
        }
        if self.min_y >= self.max_y {
            panic!("Drawer: min_y should be smaller than max_y.")
        }

        self.defined_coordinate_range_flag = true;
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
        self
    }

    ///确定图片尺寸
    pub fn build_image_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    ///确定其他信息
    /// caption为空串时表示不需要标题
    pub fn build_message(mut self, file_name: &str, caption: &str) -> Self {
        self.file_name = String::from(file_name);
        if caption != "" {
            self.caption = Some(String::from(caption));
        }
        self
    }

    ///添加结果集
    pub fn add_task(&mut self, point_vec: Vec<(f64, f64)>, color: RGBColor) {
        self.point_vec_list.push((point_vec,color));
    }

    ///进行绘图 TODO 图例
    pub fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //未限制坐标范围，则自动生成
        if self.defined_coordinate_range_flag == false {
            self.auto_calculate_coordinate_range();
        }

        let root = BitMapBackend::new(&self.file_name, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root);
        chart.set_label_area_size(LabelAreaPosition::Left, 45)
            .set_label_area_size(LabelAreaPosition::Right, 45)
            .set_label_area_size(LabelAreaPosition::Top, 45)
            .set_label_area_size(LabelAreaPosition::Bottom, 45);

        if let Some(cap) = &self.caption {
            chart.caption(cap, ("Arial", 30).into_font());
        }

        //结束构造
        let mut chart =
            chart.build_cartesian_2d(self.min_x..self.max_x, self.min_y..self.max_y)?;

        //加上网格
        chart.configure_mesh().draw()?;

        //画图
        loop {
            let pv_tuple = self.point_vec_list.pop();
            if let None = pv_tuple{
                break;
            }
            let pv_tuple=pv_tuple.unwrap();
            chart.draw_series(
                PointSeries::of_element(
                    pv_tuple.0,
                    2,
                    &pv_tuple.1,
                    &|c, s, st| {
                        return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                    },
                ),
            )?;
        }

        Ok(())
    }

    //TODO 自动生成坐标范围
    fn auto_calculate_coordinate_range(&mut self) {}
}

#[cfg(test)]
mod tests {
    use plotters::prelude::*;

    use super::*;

    #[test]
    #[ignore]
    fn test_draw() -> Result<(), Box<dyn std::error::Error>> {
        let mut drawer = Drawer::new()
            .build_coordinate_range(-10.0, 15.0, -15.0, 12.5);
        drawer.add_task(Vec::from([(0.2, 0.0), (1.0, 1.5), (2.0, 2.8), (-1.3, -3.4)]), BLUE);
        drawer.add_task(Vec::from([(-4.0, 3.9), (5.1, -6.0), (6.5, 6.5), (7.0, 7.2), (8.0, 0.1), (9.9, 9.0)]), RED);
        drawer.draw()
    }

    #[test]
    #[ignore]
    fn test_plotters() -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("散点图", ("sans-serif", 30))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..10, 0..10)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            PointSeries::of_element(
                [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8), (9, 9)],
                5,
                &BLUE,
                &|c, s, st| {
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ),
        )?;

        Ok(())
    }
}
