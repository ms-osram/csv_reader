use std::collections::HashMap;
use std::io::BufRead as _;

fn main() {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open("input.csv")
        .map(|f| std::io::BufReader::new(f))
        .unwrap();

    let mut entries = HashMap::new();
    // read the CSV file
    // do not read the header
    for line in file.lines().skip(1) {
        let line = line.unwrap();
        let mut splits = line.split(",");
        let uuid = splits.next().unwrap().to_owned();
        let value = splits.next().unwrap().parse::<f64>().unwrap();
    
        entries.entry(uuid).or_insert(vec![]).push(value);
    }

    let mut summaries = vec![];
    for (uuid, values) in entries.into_iter() {
        let mut n = 0;
        let mut cumsum = 0.;
        let mut ssq = 0.;

        // calculate mean
        for value in values.iter() {
            n += 1;
            cumsum += *value;
        }
        let mean = cumsum / n as f64;

        // calculate stdev
        for value in values.iter() {
            ssq = (mean - *value).powi(2);
        }
        let stdev = (ssq / n as f64).sqrt();

        summaries.push((uuid, n, mean, stdev));
    }

    // sort by size
    summaries.sort_unstable_by_key(|summary| -summary.1);

    for (uuid, n, mean, stdev) in summaries.into_iter() {
        println!("{}\t{}\t{}\t{}", uuid, n, mean, stdev);
    }
}
