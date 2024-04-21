use crate::model::engine::Engine;
use crate::model::product::Product;
use crate::model::purchasable::Purchasable;
use crate::model::quotation::Quotation;
use crate::model::unit::Area;
use crate::model::unit::Footprint;
use crate::model::unit::Lenght;
use crate::model::unit::LenghtType;
use crate::model::unit::UnitOfMeasure;
use crate::rules::min_quantity::MinQuantity;

pub fn mock_product() -> Product {
    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Meter, 3.0)),
        10.0,
    );
    p
}

pub fn mock_purchasable() -> Purchasable {
    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Centimeter, 15.0)),
        10.0,
    );
    let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
    pu
}

pub fn mock_quotation() -> Quotation {
    // 4
    let p = Product::new(
        "0",
        "piastrelle di ceramica",
        10.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 10.0, 10.0)),
        2.0,
    );
    // 100
    let p2 = Product::new(
        "1",
        "box doccia",
        1.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 100.0, 100.0)),
        100.0,
    );
    // 100
    let p3 = Product::new(
        "2",
        "rubinetti vintage",
        4.0,
        UnitOfMeasure::Footprint(Footprint::new(LenghtType::Centimeter, 10., 25.0, 5.0)),
        25.0,
    );
    //3.2 = 4.0 * 0.8
    let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
    //80 = 100 * 0.8
    let pu2: Purchasable = Purchasable::new(p2, 5, 1, Some(0.2));
    //80 = 100 * 0.8
    let pu3: Purchasable = Purchasable::new(p3, 5, 4, Some(0.2));
    let mut quo: Quotation = Quotation::new("0");
    quo.add_item(pu);
    quo.add_item(pu2);
    quo.add_item(pu3);
    quo
}

pub fn mock_engine() -> Engine {
    Engine::new(
        mock_quotation(),
        vec![Box::new(MinQuantity { quantity: 3 })],
    )
}
