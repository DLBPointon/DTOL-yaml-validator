use std::fs;
use std::env;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct FullDarwinYaml {
    species: String,
    specimen: String,
    projects: Vec<String>,
    data_location: String,
    primary_agp: String,
    primary_contigs: String,
    haplotigs: String,
    mito: String,
    mito_gb: String,
    mito_reference: String,
    plastid: String,
    plastid_gb: String,
    chloro_reference: String,
    pacbio_read_dir: String,
    x10x_read_dir: String,
    hic_read_dir: String,
    pacbio_read_type: String,
    hic_kit: String,
    hic: String,
    pretext: String,
    hic_map_img: String,
    kmer_spectra_img: String,
    busco_lineage: String,
    busco_gene_set_species: String,
    karyotype: String,
    karyotype_source:  String,
    ploidy: String,
    jira_queue: String,
    pipeline: Vec<String>,
    notes: String,
    stats: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct DarwinYaml {
    species: String,
    specimen: String,
    projects: Vec<String>,
    primary: String,
    agp: String,
    primary_contigs: String,
    haplotigs: String,
    mito: String,
    mito_gb: String,
    pipeline: Vec<String>,
    stats: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string(&args[2])?;

    // TODO: here it needs to id `projects:\r\n  - darwin\r\n` and
    // parse the project and use that to decide on the Yaml struc to use.

    let contents = serde_yaml::from_str::<DarwinYaml>(&file);

    println!("{:?}", file); // <-- Raw output
    //println!("{:?}", contents); <-- Yaml Deserialised
    println!("Project argument: {}", &args[1]);
    println!("File for ingesting: {}", &args[2]);
    Ok(())
}