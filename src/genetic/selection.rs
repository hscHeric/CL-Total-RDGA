use rand::seq::IteratorRandom;

use super::Population;

pub trait SelectionStrategy {
    fn select(&self, population: &Population) -> Population;
}

pub struct KTournamentSelection {
    pub tournament_size: usize,
}

impl SelectionStrategy for KTournamentSelection {
    fn select(&self, population: &Population) -> Population {
        let mut rng = rand::thread_rng();
        let mut new_individuals = Vec::with_capacity(population.size());
        let individuals = population.individuals();

        for _ in 0..population.size() {
            // Seleciona índices aleatórios para o torneio
            let indices: Vec<usize> =
                (0..individuals.len()).choose_multiple(&mut rng, self.tournament_size);

            // Determina o melhor indivíduo no torneio
            let best_index = indices
                .iter()
                .max_by_key(|&&i| individuals[i].fitness())
                .unwrap();

            // Clona o melhor indivíduo para a nova população
            new_individuals.push(individuals[*best_index].clone());
        }

        Population::new_from_individuals(new_individuals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{genetic::Chromosome, graph::SimpleGraph};

    #[test]
    fn test_k_tournament_selection() {
        let mut graph = SimpleGraph::new();

        graph.add_vertex(0).unwrap();
        graph.add_vertex(1).unwrap();
        graph.add_vertex(2).unwrap();
        graph.add_vertex(3).unwrap();

        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        graph.add_edge(3, 0).unwrap();

        let best_chromosome = Chromosome::new(vec![2, 2, 2, 2]); // fitness = 8
        let mid_chromosome = Chromosome::new(vec![1, 2, 1, 2]); // fitness = 6
        let worst_chromosome = Chromosome::new(vec![1, 1, 1, 1]); // fitness = 4

        let mut initial_pop = Vec::new();
        for _ in 0..5 {
            initial_pop.push(worst_chromosome.clone());
        }
        for _ in 0..3 {
            initial_pop.push(mid_chromosome.clone());
        }
        for _ in 0..2 {
            initial_pop.push(best_chromosome.clone());
        }

        let population = Population::new_from_individuals(initial_pop);

        let tournament = KTournamentSelection { tournament_size: 3 };

        let selected_pop = tournament.select(&population);

        assert_eq!(
            selected_pop.size(),
            population.size(),
            "Selected population should maintain the same size"
        );

        for individual in selected_pop.individuals() {
            assert!(
                individual.is_valid_to_total_roman_domination(&graph),
                "All selected individuals must be valid for total roman domination"
            );
        }

        let has_good_individual = selected_pop.individuals().iter().any(|ind| {
            let genes = ind.genes();
            genes.iter().sum::<u8>() >= 6 // at least as good as mid_chromosome
        });

        assert!(
            has_good_individual,
            "Selected population should contain at least one good individual"
        );

        for individual in selected_pop.individuals() {
            assert_eq!(
                individual.genes().len(),
                graph.vertex_count(),
                "Each chromosome should have genes equal to the number of vertices"
            );
        }
    }
}
