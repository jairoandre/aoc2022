use rand::prelude::*;
use rand::seq::SliceRandom;

// Define a trait for objects that can be evaluated by the genetic algorithm
trait Fitness {
    fn fitness(&self) -> f64;
}

// Define a struct for representing a population of individuals
struct Population<T: Fitness> {
    individuals: Vec<T>,
    population_size: usize,
}

impl<T: Fitness> Population<T> {
    // Create a new population with random individuals
    fn new(population_size: usize) -> Population<T> {
        let mut individuals = Vec::with_capacity(population_size);
        for _ in 0..population_size {
            individuals.push(T::random());
        }
        Population {
            individuals,
            population_size,
        }
    }

    // Evaluate the fitness of each individual in the population
    fn evaluate_fitness(&mut self) {
        self.individuals.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
    }

    // Select a subset of individuals from the population using tournament selection
    fn select(&self, k: usize) -> Vec<T> {
        let mut rng = rand::thread_rng();
        let mut selected = Vec::with_capacity(k);
        for _ in 0..k {
            let mut tournament = Vec::with_capacity(k);
            for _ in 0..k {
                tournament.push(self.individuals.choose(&mut rng).unwrap());
            }
            tournament.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
            selected.push(tournament[0].clone());
        }
        selected
    }

    // Evolve the population by applying crossover and mutation
    fn evolve(&mut self, crossover_rate: f64, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
        let mut next_gen = Vec::with_capacity(self.population_size);
        while next_gen.len() < self.population_size {
            // Select two individuals from the population using tournament selection
            let parents = self.select(4);
            let parent1 = &parents[0];
            let parent2 = &parents[1];

            // Apply crossover to produce two offspring
            let offspring1 = T::crossover(parent1, parent2, crossover_rate);
            let offspring2 = T::crossover(parent1, parent2, crossover_rate);

            // Apply mutation to the offspring
            let offspring1 = T::mutate(&offspring1, mutation_rate);
            let offspring2 = T::mutate(&offspring2, mutation_rate);

            // Add the offspring to the next generation
            next_gen.push(offspring1);
            next_gen.push(offspring2);
        }
        self.individuals = next_gen;
    }
}