// loads and processes pokedex.csv data
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Pokemon {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "hp")]
    pub hp: f64,

    #[serde(rename = "attack")]
    pub attack: f64,

    #[serde(rename = "defense")]
    pub defense: f64,

    #[serde(rename = "s_attack")]
    pub sp_atk: f64,

    #[serde(rename = "s_defense")]
    pub sp_def: f64,

    #[serde(rename = "speed")]
    pub speed: f64,
}

// returns a vector of all base stats as a normalized vector
impl Pokemon {
    pub fn base_stats(&self) -> Vec<f64> {
        vec![self.hp, self.attack, self.defense, self.sp_atk, self.sp_def, self.speed]
    }
}

// normalize the stats of all pokemon to the range [0.0, 1.0]
pub fn normalize_pokemons(pokemons: &mut Vec<Pokemon>) {
    if pokemons.is_empty() {
        return;
    }

    let mut max_hp = 0.0;
    let mut max_attack = 0.0;
    let mut max_defense = 0.0;
    let mut max_sp_atk = 0.0;
    let mut max_sp_def = 0.0;
    let mut max_speed = 0.0;

    // first pass: find maximums
    for p in pokemons.iter() {
        if p.hp > max_hp { max_hp = p.hp; }
        if p.attack > max_attack { max_attack = p.attack; }
        if p.defense > max_defense { max_defense = p.defense; }
        if p.sp_atk > max_sp_atk { max_sp_atk = p.sp_atk; }
        if p.sp_def > max_sp_def { max_sp_def = p.sp_def; }
        if p.speed > max_speed { max_speed = p.speed; }
    }

    // second pass: scale each stat
    for p in pokemons.iter_mut() {
        p.hp /= max_hp;
        p.attack /= max_attack;
        p.defense /= max_defense;
        p.sp_atk /= max_sp_atk;
        p.sp_def /= max_sp_def;
        p.speed /= max_speed;
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
