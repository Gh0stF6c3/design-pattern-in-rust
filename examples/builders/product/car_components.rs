#[allow(unused)]
#[derive(Debug,Copy,Clone)]
pub enum CarType {
    CityCar,
    SportsCar,
    Suv
}

#[allow(unused)]
#[derive(Debug)]
pub enum Transmission {
    SingleSpeed,
    Manual,
    Automatic,
    SemiAutomatic
}

#[allow(unused)]
#[derive(Debug)]
pub struct Engine {
    volume: f64,
    mileage: f64,
    started: bool
}

#[allow(unused)]
impl Engine {
    pub fn new(volume: f64, mileage: f64) -> Self {
        Engine {
            volume,
            mileage,
            started: false
        }
    }

    pub fn on(&mut self) {
        self.started = true;
    }

    pub fn off(&mut self) {
        self.started = false;
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn mileage(&self) -> f64 {
        self.mileage
    }

    pub fn go(&mut self,mileage: f64) {
        if self.started() {
            self.mileage += mileage;
        } else {
            println!("启程失败，引擎未启动!")
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct GpsNavigator {
    route: String
}

#[allow(unused)]
impl GpsNavigator {
    pub fn new() -> Self {
        Self::from_route(
            "GPS坐标: xxx-xxx-xxx 1-1-1-1".into()
        )
    }

    pub fn from_route(route: String) -> Self {
        Self {route}
    }

    pub fn route(&self) -> &String {
        &self.route
    }
}