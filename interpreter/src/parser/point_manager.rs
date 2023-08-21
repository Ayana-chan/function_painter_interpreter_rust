pub struct PointManager {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,

    point_storage: Option<Vec<(f64, f64)>>, //可以低消耗地共享

    var_origin: (f64, f64),
    var_scale: (f64, f64),
    var_rot: f64,
    //提前运算以加速
    var_rot_sin: f64,
    var_rot_cos: f64,
}

impl PointManager {
    pub fn new() -> Self {
        Self {
            min_x: -8000.0,
            max_x: 8000.0,
            min_y: -5000.0,
            max_y: 5000.0,

            point_storage: Some(Vec::new()),

            var_origin: (0.0, 0.0),
            var_scale: (1.0, 1.0),
            var_rot: 0.0,
            var_rot_sin: 0.0,
            var_rot_cos: 1.0,
        }
    }

    ///添加一个点
    pub fn add_point(&mut self, mut new_point: (f64, f64)) -> Result<(), ()> {
        //计算点位置
        new_point.0 *= self.var_scale.0;
        new_point.1 *= self.var_scale.1;
        let temp_x = &new_point.0 * &self.var_rot_cos - &new_point.1 * &self.var_rot_sin;
        let temp_y = &new_point.0 * &self.var_rot_sin + &new_point.1 * &self.var_rot_cos;
        new_point.0 = temp_x+self.var_origin.0;
        new_point.1 = temp_y+self.var_origin.1;

        println!("Debug: Add Point: {:?}", new_point);

        if new_point.0 < self.min_x || new_point.0 > self.max_x
            || new_point.1 < self.min_y || new_point.1 > self.max_y {
            //越界，无视该点
            return Err(());
        }
        self.extract_mut_point_storage().push(new_point);
        Ok(())
    }

    fn extract_mut_point_storage(&mut self) -> &mut Vec<(f64, f64)> {
        if let Some(ps) = &mut self.point_storage {
            return ps;
        } else {
            panic!("PointManager: point_storage is None.")
        }
    }

    pub fn move_point_storage(&mut self) -> Vec<(f64, f64)> {
        self.point_storage.take().unwrap()
    }

    pub fn set_coordinate_range(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        if self.min_x >= self.max_x {
            panic!("Drawer: min_x should be smaller than max_x.")
        }
        if self.min_y >= self.max_y {
            panic!("Drawer: min_y should be smaller than max_y.")
        }

        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
    }

    pub fn set_var_origin(&mut self, var_origin: (f64, f64)) {
        self.var_origin = var_origin;
    }
    pub fn set_var_scale(&mut self, var_scale: (f64, f64)) {
        self.var_scale = var_scale;
    }
    pub fn set_var_rot(&mut self, var_rot: f64) {
        self.var_rot = var_rot;
        //预计算
        self.var_rot_sin = var_rot.sin();
        self.var_rot_cos = var_rot.cos();
    }

    // pub fn min_x(&self) -> f64 {
    //     self.min_x
    // }
    // pub fn max_x(&self) -> f64 {
    //     self.max_x
    // }
    // pub fn min_y(&self) -> f64 {
    //     self.min_y
    // }
    // pub fn max_y(&self) -> f64 {
    //     self.max_y
    // }
    // pub fn var_origin(&self) -> &(f64, f64) {
    //     &self.var_origin
    // }
    // pub fn var_scale(&self) -> &(f64, f64) {
    //     &self.var_scale
    // }
    // pub fn var_rot(&self) -> f64 {
    //     self.var_rot
    // }
}






