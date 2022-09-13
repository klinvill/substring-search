use substring_search::has_substring;

fn main() {
    let s1 = std::fs::read_to_string("./data/Tolstoy/war_and_peace_tolstoy.txt").unwrap();
    let s2 = std::fs::read_to_string("./data/Tolstoy/anna_karenina_tolstoy.txt").unwrap();
    // let s1 = std::fs::read_to_string("./data/genomes/bacterial_genome_1.txt").unwrap();
    // let s2 = std::fs::read_to_string("./data/genomes/monkeypox-genome.txt").unwrap();
    println!("Common substring?: {}", has_substring(&s1, &s2, 20));
}
