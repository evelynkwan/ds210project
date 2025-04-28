// basic tests for project components

#[cfg(test)]
mod tests {
    use crate::data::Pokemon;
    use crate::graph::Graph;

    fn dummy_pokemon() -> Vec<Pokemon> {
        vec![
            Pokemon { name: "Bulbasaur".to_string(), hp: 45.0, attack: 49.0, defense: 49.0, sp_atk: 65.0, sp_def: 65.0, speed: 45.0, primary_type: "Grass".to_string() },
            Pokemon { name: "Charmander".to_string(), hp: 39.0, attack: 52.0, defense: 43.0, sp_atk: 60.0, sp_def: 50.0, speed: 65.0, primary_type: "Fire".to_string() },
        ]
    }

    #[test]
    fn test_distance_nonzero() {
        let pokemons = dummy_pokemon();
        let d = Graph::distance(&pokemons[0], &pokemons[1]);
        assert!(d > 0.0);
    }

    #[test]
    fn test_knn_graph_build() {
        let pokemons = dummy_pokemon();
        let graph = Graph::build_knn_graph(&pokemons, 1);
        assert_eq!(graph.adjacency.len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Pokemon};
    use crate::data::load_pokemon_data;
    use csv::ReaderBuilder;
    use std::io::Cursor;

    #[test]
    fn test_load_pokemon_data_from_string() {
        let data = "\
name,hp,attack,defense,sp_atk,sp_def,speed,primary_type
Bulbasaur,45,49,49,65,65,45,Grass
Charmander,39,52,43,60,50,65,Fire
";

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(data));

        let mut pokemons: Vec<Pokemon> = Vec::new();
        for result in rdr.deserialize() {
            let record: Pokemon = result.expect("Failed to parse record");
            pokemons.push(record);
        }

        assert_eq!(pokemons.len(), 2);
        assert_eq!(pokemons[0].name, "Bulbasaur");
        assert_eq!(pokemons[1].primary_type, "Fire");
    }
}