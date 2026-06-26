use serde::{ Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Asset {
  pub id: i64,
  pub name: String,
  pub unit_value: f64,
}
