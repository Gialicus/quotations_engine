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
    let product: Product = Product::new(
        "cavi usb rame x5 Gialix".into(),
        "cavi usb in rame lunghi 3 metri",
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Meter, 3.0)),
    );
    product
}

pub fn mock_purchasable() -> Purchasable {
    let product: Product = Product::new(
        "cavi usb rame x5 Gialix".into(),
        "cavi usb in rame lunghi 15 centimetri",
        5.0,
        UnitOfMeasure::Lenght(Lenght::new(LenghtType::Centimeter, 15.0)),
    );
    let price = Price::new("0", 10.0, 0.5, 0);
    //20 /S 10
    let pu: Purchasable = Purchasable::new(product, price, 2, Vec::new());
    pu
}

pub fn mock_quotation() -> Quotation {
    let p = Product::new(
        "piastrelle di ceramica",
        "cavi usb in rame lunghi 10 centimetri",
        10.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 10.0, 10.0)),
    );
    let p2 = Product::new(
        "box doccia",
        "box doccia 1x1 m",
        1.0,
        UnitOfMeasure::Area(Area::new(LenghtType::Centimeter, 100.0, 100.0)),
    );
    let p3 = Product::new(
        "rubinetti vintage",
        "cose da bagno",
        4.0,
        UnitOfMeasure::Footprint(Footprint::new(LenghtType::Centimeter, 10., 25.0, 5.0)),
    );
    //40 /S 20
    let pu: Purchasable = Purchasable::new(
        p.clone(),
        Price::new(p.id.as_str(), 20.0, 0.5, 0),
        2,
        Vec::new(),
    );
    //200 /S 100
    let pu2: Purchasable = Purchasable::new(
        p2.clone(),
        Price::new(p2.id.as_str(), 200.0, 0.5, 0),
        1,
        Vec::new(),
    );
    //200 /S 100
    let pu3: Purchasable = Purchasable::new(
        p3.clone(),
        Price::new(p3.id.as_str(), 50.0, 0.5, 0),
        4,
        Vec::new(),
    );
    let mut quo: Quotation = Quotation::new();
    quo.add_item(pu);
    quo.add_item(pu2);
    quo.add_item(pu3);
    quo
}
