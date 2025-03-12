use crate::product::{car::Car, car_components::{CarType, Engine, GpsNavigator, Transmission}};

pub const DEFAULT_FUEL: f64 = 5f64;


/// 构建器用于拆分一个目标对象的构建步骤，通过`set_*`这样类似的设置函数对构建器的参数进行
/// 值填充，值得注意的是不需要对每个参数值进行填充，只需要填充`build`函数所需要的参数即可。
/// 对build构建函数的错误处理及提示就变得尤为重要，否则我们将得到错误的对象

pub trait Builder {
    type OutputType;

    fn set_car_type(&mut self,car_type: CarType);
    fn set_seats(&mut self,seats: u16);
    fn set_engine(&mut self,engine: Engine);
    fn set_transmission(&mut self,transmission: Transmission);
    fn set_gps_navigator(&mut self,gps_navigator: GpsNavigator);

    fn build(self) -> Self::OutputType;
}




#[derive(Debug,Default)]
pub struct CarBuilder {
    car_type: Option<CarType>,
    engine: Option<Engine>,
    gps_navigator: Option<GpsNavigator>,
    transmission: Option<Transmission>,
    seats: Option<u16>
}

impl Builder for CarBuilder {
    type OutputType = Car;

    fn set_car_type(&mut self,car_type: CarType) {
        self.car_type = Some(car_type);
    }

    fn set_engine(&mut self,engine: Engine) {
        self.engine = Some(engine);
    }

    fn set_transmission(&mut self,transmission: Transmission) {
        self.transmission = Some(transmission);
    }

    fn set_gps_navigator(&mut self,gps_navigator: GpsNavigator) {
        self.gps_navigator = Some(gps_navigator);
    }
    
    fn set_seats(&mut self,seats: u16) {
        self.seats = Some(seats);
    }

    fn build(self) -> Self::OutputType {
        Car::new(
            self.car_type.expect("未设置车辆类型"), 
            self.engine.expect("未设置引擎类型"), 
            self.gps_navigator.expect("未设置GPS导航"), 
            self.transmission.expect("未设置驱动类型"), 
            self.seats.expect("未设置车辆座位数"), 
            DEFAULT_FUEL
        )
    }
}