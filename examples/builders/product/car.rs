use super::car_components::{CarType, Engine, GpsNavigator, Transmission};

#[allow(unused)]
#[derive(Debug)]
pub struct Car {
    car_type: CarType,
    engine: Engine,
    gps_navigator: GpsNavigator,
    tranmission: Transmission,
    seats: u16,
    fuel: f64
}

#[allow(unused)]
impl Car {
    pub fn new(
        car_type: CarType,
        engine: Engine,
        gps_navigator: GpsNavigator,
        tranmission: Transmission,
        seats: u16,
        fuel: f64
    ) -> Self {
        Self {
            car_type,
            engine,
            gps_navigator,
            tranmission,
            seats,
            fuel
        }
    }

    pub fn car_type(&self) -> CarType {
        self.car_type
    }

    pub fn fuel(&self) -> f64 {
        self.fuel
    }

    pub fn set_fuel(&mut self,fuel: f64) {
        self.fuel = fuel
    }

    pub fn seats(&self) -> u16 {
        self.seats
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn tranmission(&self) -> &Transmission {
        &self.tranmission
    }

    pub fn gps_navigator(&self) -> &GpsNavigator {
        &self.gps_navigator
    }
}