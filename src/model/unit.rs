use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LenghtType {
    Meter,
    Centimeter,
    Millimeter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VolumeType {
    Liter,
    Centiliter,
    Milliliter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MassType {
    Kilogram,
    Gram,
    Centigram,
    Milligram,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TimeType {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    Lenght((LenghtType, f64)),
    Area((LenghtType, (f64, f64))),
    Footprint((LenghtType, (f64, f64, f64))),
    Volume((VolumeType, (f64, f64, f64))),
    Mass((MassType, f64)),
    Time(TimeType, f64),
}
