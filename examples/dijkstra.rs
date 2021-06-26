use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{App, Arg};
use pheap::graph::Graph;

fn main() {
    let matches = App::new("simct benchmark")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("Path to a DIMACS file."),
        )
        .arg(
            Arg::with_name("lib")
                .long("lib")
                .takes_value(true)
                .required(true)
                .help("The library to be used to solve the shortest path problem. Options: pheap | fast_paths."),
        )
        .arg(
            Arg::with_name("runs")
                .long("runs")
                .takes_value(true)
                .default_value("5")
                .help("Number of runs for search query."),
        )
        .get_matches();

    let filepath = match matches.value_of("file") {
        Some(fp) => fp,
        None => std::process::exit(1),
    };

    let runs = matches
        .value_of("runs")
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    match matches.value_of("lib") {
        Some(lib) => match lib {
            "pheap" => graph(filepath, runs),
            "fast_paths" => fast_paths(filepath, runs),
            _ => std::process::exit(1),
        },
        None => std::process::exit(1),
    };
}

fn graph(filepath: &str, runs: usize) {
    println!("> Load file: {}", filepath);

    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    let mut n_nodes = 0;
    let mut n_edges = 0;

    for _ in 0..7 {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if !line.is_empty() && line.starts_with('p') {
            let s = line.trim().split_whitespace().collect::<Vec<&str>>();
            n_nodes = s[2].parse::<usize>().unwrap();
            n_edges = s[3].parse::<usize>().unwrap();
        }
    }

    let mut g = Graph::<u32>::with_size(n_nodes, n_edges);

    for line in reader.lines() {
        let line = line.unwrap();
        let s = line.trim().split_whitespace().collect::<Vec<&str>>();
        let node1 = s[1].parse::<usize>().unwrap() - 1;
        let node2 = s[2].parse::<usize>().unwrap() - 1;
        let weight = s[3].parse::<u32>().unwrap();

        g.add_weighted_edges(node1, node2, weight);
    }

    println!("> Graph created.");

    let mut durations = Vec::with_capacity(runs);

    for ii in 0..runs {
        println!("> Run {}/{}", ii + 1, runs);
        let start = std::time::Instant::now();
        let _ = g.sssp_dijkstra(10_000, None);
        let end = std::time::Instant::now() - start;
        println!(
            "> Time taken to solve the problem: {} (ms)",
            end.as_millis()
        );
        durations.push(end.as_millis());
    }

    let avg = durations.iter().sum::<u128>() as usize;
    println!("Average time: {} (ms)", avg / runs);
}

fn fast_paths(filepath: &str, runs: usize) {
    println!("> Load file: {}", filepath);

    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    let mut n_nodes = 0;
    let mut n_edges = 0;

    for _ in 0..7 {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if !line.is_empty() && line.starts_with('p') {
            let s = line.trim().split_whitespace().collect::<Vec<&str>>();
            n_nodes = s[2].parse::<usize>().unwrap();
            n_edges = s[3].parse::<usize>().unwrap();
        }
    }

    let mut g = Graph::<u32>::with_size(n_nodes, n_edges);

    for line in reader.lines() {
        let line = line.unwrap();
        let s = line.trim().split_whitespace().collect::<Vec<&str>>();
        let node1 = s[1].parse::<usize>().unwrap() - 1;
        let node2 = s[2].parse::<usize>().unwrap() - 1;
        let weight = s[3].parse::<u32>().unwrap();

        g.add_weighted_edges(node1, node2, weight);
    }

    println!("> Graph created.");

    let mut durations = Vec::with_capacity(runs);

    for ii in 0..runs {
        println!("> Run {}/{}", ii + 1, runs);
        let start = std::time::Instant::now();
        let _ = g.sssp_dijkstra(10_000, None);
        let end = std::time::Instant::now() - start;
        println!(
            "> Time taken to solve the problem: {} (ms)",
            end.as_millis()
        );
        durations.push(end.as_millis());
    }

    let avg = durations.iter().sum::<u128>() as usize;
    println!("Average time: {} (ms)", avg / runs);
}
