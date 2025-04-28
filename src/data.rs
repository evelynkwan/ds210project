// loads and processes pokedex.csv data
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Pokemon {
    pub name: String,
    pub hp: f64,
    pub attack: f64,
    pub defense: f64,
    pub sp_atk: f64,
    pub sp_def: f64,
    pub speed: f64,
    pub primary_type: String,
}

// returns a vector of all base stats as a normalized vector
impl Pokemon {
    pub fn base_stats(&self) -> Vec<f64> {
        vec![self.hp, self.attack, self.defense, self.sp_atk, self.sp_def, self.speed]
    }
}

// loads pokemon data from pokedex.csv 
pub fn load_pokemon_data<P: AsRef<Path>>(path: P) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut pokemons = Vec::new();
    for result in rdr.deserialize() {
        let record: Pokemon = result?;
        pokemons.push(record);
    }
    Ok(pokemons)
}
