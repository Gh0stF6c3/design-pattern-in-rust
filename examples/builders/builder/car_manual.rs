use crate::product::{car_components::{CarType, Engine, GpsNavigator, Transmission}, car_manual::Manual};

use super::car::Builder;

#[derive(Debug,Default)]
pub struct CarManualBuilder {
    car_type: Option<CarType>,
    engine: Option<Engine>,
    gps_navigator: Option<GpsNavigator>,
    transmission: Option<Transmission>,
    seats: Option<u16>
}

impl Builder for CarManualBuilder {
    type OutputType = Manual;

    fn set_car_type(&mut self,car_type: CarType) {
        self.car_type = Some(car_type);
    }

    fn set_engine(&mut self,engine: Engine) {
        self.engine = Some(engine);
    }

    fn set_gps_navigator(&mut self,gps_navigator: GpsNavigator) {
        self.gps_navigator = Some(gps_navigator);
    }

    fn set_transmission(&mut self,transmission: Transmission) {
        self.transmission = Some(transmission);
    }

    fn set_seats(&mut self,seats: u16) {
        self.seats = Some(seats);
    }

    fn build(self) -> Self::OutputType {
        Manual::new(
            self.car_type.expect("未设置车辆类型"), 
            self.engine.expect("未设置引擎类型"), 
            self.gps_navigator, 
            self.transmission.expect("未设置驱动类型"), 
            self.seats.expect("未设置车辆座位数"),
        )
    }
}