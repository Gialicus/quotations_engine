use crate::model::price::Price;
use crate::model::product::Product;
use crate::model::purchasable::Purchasable;
use crate::model::quotation::Quotation;
use crate::model::unit::Area;
use crate::model::unit::Footprint;
use crate::model::unit::Lenght;
use crate::model::unit::LenghtType;
use crate::model::unit::UnitOfMeasure;

pub fn mock_product() -> Product {
    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Meter, 3.0)),
    );
    p
}

pub fn mock_purchasable() -> Purchasable {
    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Centimeter, 15.0)),
    );
    let price = Price::new(10.0, 0.5, 0);
    //20 /S 10
    let pu: Purchasable = Purchasable::new(p, price, 2);
    pu
}

pub fn mock_quotation() -> Quotation {
    let p = Product::new(
        "0",
        "piastrelle di ceramica",
        10.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 10.0, 10.0)),
    );
    let p2 = Product::new(
        "1",
        "box doccia",
        1.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 100.0, 100.0)),
    );
    let p3 = Product::new(
        "2",
        "rubinetti vintage",
        4.0,
        UnitOfMeasure::Footprint(Footprint::new(LenghtType::Centimeter, 10., 25.0, 5.0)),
    );
    //40 /S 20
    let pu: Purchasable = Purchasable::new(p, Price::new(20.0, 0.5, 0), 2);
    //200 /S 100
    let pu2: Purchasable = Purchasable::new(p2, Price::new(200.0, 0.5, 0), 1);
    //200 /S 100
    let pu3: Purchasable = Purchasable::new(p3, Price::new(50.0, 0.5, 0), 4);
    let mut quo: Quotation = Quotation::new("0");
    quo.add_item(pu);
    quo.add_item(pu2);
    quo.add_item(pu3);
    quo
}
