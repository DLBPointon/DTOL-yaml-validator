use serde::Deserialize;

//#[derive(Debug, Serialize, Deserialize)]
/* struct DtolConfig {
    species: String,
    specimen: String,
    projects: String,
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
    10x_read_dir: String,
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
    pipeline: String,
    notes: String,
    stats: String,
} */

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

fn main() -> Result<(), serde_yaml::Error> {
    let input = include_str!("../test-yaml/idCalVici1-truncated.yaml");
    let contents = serde_yaml::from_str::<DarwinYaml>(&input);
    println!("{:?}", contents);
    Ok(())
}