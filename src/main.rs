use std::fs;
use std::env;
use serde::Deserialize;
use regex::Regex;

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

fn darwin_function() {
    println!("Hi im Darwin")
}

fn asg_function() {
    println!("Hi im ASG")
}

fn genomeark_function()  {
    println!("Hi im genomeArk")
}

fn erga_function()  {
    println!("Hi im ERGA")
}

fn faculty_function() {
    println!("Hi im faculty")
}

fn generate_template() {
    println!("Do you need some help?")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Master list of allowed projects
    let valid_projects = ["darwin", "asg", "genomeark", "erga", "faculty"];

    // Convert CLI args into somthing usable
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[2])?;
    if &args[3] == "-gen" {
        generate_template()
    }
 
    // Use raw data to find project - Will fail if more than one
    let regex = Regex::new(r"projects:\r\n..-.([a-z]*)").unwrap();
    let regexed_project = regex.captures(&file).unwrap().get(1).map_or("", |m| m.as_str());
    if !valid_projects.contains(&regexed_project) {
        panic!("Not a valid project! \nCheck format and make sure you pick a valid option.\n {:?}", valid_projects)
    }

    println!("CLI reports: {} | YAML reports: {}", &args[1], regexed_project);

    match &*args[1] {
        "darwin" => darwin_function(),
        "asg" => asg_function(),
        "genomeark" => genomeark_function(),
        "erga" => erga_function(),
        "faculty" => faculty_function(),
        _ => panic!("What is {}, I don't know that one!", &args[1]),
    }

    Ok(())
}

    // TODO: here it needs to id `projects:\r\n  - darwin\r\n` and
    // parse the project and use that to decide on the Yaml struc to use.

    //let contents = serde_yaml::from_str::<DarwinYaml>(&file);

    //    println!("{:?}", file); // <-- Raw output
  //  //println!("{:?}", contents); <-- Yaml Deserialised
    //println!("Project argument: {}", &args[1]);
    //println!("File for ingesting: {}", &args[2]);
    //Ok(())
//}