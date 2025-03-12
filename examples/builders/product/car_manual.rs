use std::fmt::Display;

use super::car_components::{CarType, Engine, GpsNavigator, Transmission};

pub struct Manual {
    car_type: CarType,
    engine: Engine,
    gps_navigator: Option<GpsNavigator>,
    transmission: Transmission,
    seats: u16,
}

impl Manual {
    pub fn new(
        car_type: CarType,
        engine: Engine,
        gps_navigator: Option<GpsNavigator>,
        transmission: Transmission,
        seats: u16,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}

impl Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"车辆类型: {:?}",self.car_type)?;
        writeln!(f,"座位数: {:?}",self.seats)?;
        writeln!(f,"引擎状态: {{{:?}}} {{{:?}}}",self.engine.volume(),self.engine.mileage())?;
        writeln!(f,"变速器: {:?}",self.transmission)?;
        match self.gps_navigator {
            Some(_) => writeln!(f,"GPS定位: 已设定目标"),
            _ => writeln!(f,"GPS定位: N/A")
        }?;
        Ok(())
    }
}