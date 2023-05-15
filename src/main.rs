//use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
//use serde::Deserialize;
//use serde_yaml::Value;

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

fn main() -> std::io::Result<()> {

    let mut file = match File::open("test-yaml/idCalVici1.yaml") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("Not Found!");
                    return Ok(());
                }
                _ => return Err(error),
            }
        }
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("File contents: {:?}", contents);

    Ok(())
}
