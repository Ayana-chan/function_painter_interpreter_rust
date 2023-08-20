use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct PointManager {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,

    point_storage: Rc<RefCell<Vec<(f64, f64)>>>, //可以低消耗地共享

    var_origin: (f64, f64),
    var_scale: (f64, f64),
    var_rot: f64,
}

impl PointManager {
    pub fn new() -> Self {
        //TODO 自定义范围大小
        Self {
            min_x: -800.0,
            max_x: 800.0,
            min_y: -500.0,
            max_y: -500.0,

            point_storage: Rc::new(RefCell::new(Vec::new())),

            var_origin: (0.0, 0.0),
            var_scale: (1.0, 1.0),
            var_rot: 0.0,
        }
    }

    ///添加一个点
    pub fn add_point(&mut self, new_point: (f64, f64)) -> Result<(), ()> {
        if new_point.0 < self.min_x || new_point.0 > self.max_x
            || new_point.1 < self.min_y || new_point.1 > self.max_y {
            //越界，无视该点
            return Err(());
        }
        self.get_mut_point_storage().push(new_point);
        Ok(())
    }

    fn get_mut_point_storage(&self) -> RefMut<Vec<(f64, f64)>> {
        self.point_storage.borrow_mut()
    }

    pub fn get_point_storage(&self) -> Rc<RefCell<Vec<(f64, f64)>>> {
        self.point_storage.clone()
    }

    pub fn set_var_origin(&mut self, var_origin: (f64, f64)) {
        self.var_origin = var_origin;
    }
    pub fn set_var_scale(&mut self, var_scale: (f64, f64)) {
        self.var_scale = var_scale;
    }
    pub fn set_var_rot(&mut self, var_rot: f64) {
        self.var_rot = var_rot;
    }

    pub fn min_x(&self) -> f64 {
        self.min_x
    }
    pub fn max_x(&self) -> f64 {
        self.max_x
    }
    pub fn min_y(&self) -> f64 {
        self.min_y
    }
    pub fn max_y(&self) -> f64 {
        self.max_y
    }
    pub fn var_origin(&self) -> &(f64, f64) {
        &self.var_origin
    }
    pub fn var_scale(&self) -> &(f64, f64) {
        &self.var_scale
    }
    pub fn var_rot(&self) -> f64 {
        self.var_rot
    }
}






