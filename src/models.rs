use serde::{ Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
  pub id: i32,
  pub name: String,
  pub unit_value: f64,
}
