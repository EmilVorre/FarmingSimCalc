use std::fmt;

#[derive(Debug)]
enum Crops {
  Empty,
  Wheat,
  Barley,
  Canola,
  Oats,
  Corn,
  Sunflower,
  Soybeans,
  Potatoes,
  Rice,
  DryRice,
  SugarBeet,
  SugarCane,
  Cotton,
  Sorghum,
  Graps,
  Olives,
  Poplar,
  RedBeet,
  Carrot,
  Parsnip,
  GreenBean,
  Peas,
  Spinach,
  Grass,
}

impl fmt::Display for Crops {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
enum Animals {
  Bees,
  Chickens,
  Cows,
  Goats,
  Horses,
  Pigs,
  Sheeps,
  WaterBuffalo,
}

#[derive(Debug)]
enum Maps {
  RiverbendSprings,
  HutanPantai,
  Zielonka,
}

#[derive(Debug)]
enum Products {
  Flour,
  Bread,

}

impl fmt::Display for Products {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}
