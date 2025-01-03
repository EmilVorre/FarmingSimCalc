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
  Grapes,
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

/* I dont think i will make it so that the player can choose the map because that will be a hassle with the data loading process aka i dont want to make the start data for the maps its so time consumming to load the map and look at every field on it to log it, so just not gonna do it for now RiverSprings for life
#[derive(Debug)]
enum Maps {
  RiverbendSprings,
  HutanPantai,
  Zielonka,
}
*/

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

#[derive(Debug)]
enum Months {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December,
}

impl fmt::Display for Months {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
enum Buildings {
  Bakery,
  Mill,
  Dairy,
}

impl fmt::Display for Buildings {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}