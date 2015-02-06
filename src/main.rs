use std::old_io::{File, BufferedReader};
use std::iter::IteratorExt;
use std::str::{Str, StrExt};
use std::rand::{Rng, thread_rng};
use std::num::Float;

pub mod numvector;
use numvector::vector;

fn cost_function(clusters: &[usize], data: &[vector], means: &[vector]) -> f32 {
    let mut res = 0.0;
    let m = data.len();

    for i in range(0, m) {
        res += (&data[i] - &means[clusters[i]]).squared_length();
    }
    res
}

fn set_current_nearest(clusters: &mut [usize], data: &[vector], means: &[vector]) {
   
    for c in clusters.iter_mut() {
        *c = 0;
    }

    for (i, x) in data.iter().enumerate() {
        for (j, m) in means.iter().enumerate() {
            if (x - &means[clusters[i]]).squared_length() > (x - m).squared_length() {
                clusters[i] = j;
            }
        }
    }
}

fn update_means(clusters: &[usize], data: &[vector], means: &mut [vector]) {
    let mut cnt = Vec::<u32>::new();
    cnt.resize(means.len(), 0);

    for m in means.iter_mut() {
        *m = vector::zero(m.size());
    }

    for (i, c) in clusters.iter().enumerate() {
        cnt[*c] += 1;
        means[*c] = &means[*c] + &data[i];
    }

    for (i, m) in means.iter_mut().enumerate() {
        if cnt[i] != 0 {
            *m = m * (1.0 / (cnt[i] as f32));
        }
    }
}

fn main() {
    let path = Path::new("dataset.txt");
    let file = &mut BufferedReader::new(match File::open(&path) {
        Err(e) => panic!("Can't open file {}: {}", path.display(), e.desc),
        Ok(file) => file
    });

    let mut it = file.lines();

    let n = match it.next() {
        None => panic!("Wrong format: first line is number of dimensions"),
        Some(n) => match n.ok().unwrap().as_slice().trim().parse::<usize>().ok() {
            None => panic!("Wrong format: first line is number of dimensions"),
            Some(n) => n
        }
    };

    let m = match it.next() {
        None => panic!("Wrong format: second number is number of examples"),
        Some(m) => match m.ok().unwrap().as_slice().trim().parse::<usize>().ok() {
            None => panic!("Wrong format: second number is number of examples"),
            Some(m) => m
        }
    };

    let k = match it.next() {
        None => panic!("Wrong format: third number is number of clusters"),
        Some(k) => match k.ok().unwrap().as_slice().trim().parse::<usize>().ok() {
            None => panic!("Wrong format: third number is number of clusters"),
            Some(k) => k
        }
    };

    let mut examples = Vec::new();

    loop {
        let line = match it.next() {
            None => { break }
            Some(s) => s.ok().unwrap()
        };

        let example = line.trim().split_str(" ").map(|s| { match s.as_slice().trim().parse::<f32>().ok() {
            Some(f) => f,
            None => panic!("Wrong format: line must contain only numbers")
        }}).collect::<Vec<f32>>();

        match example.len() {
            x if x == 0 => { break },
            x if x != n => panic!("Wrong format, example must have {} dimensions", n),
            _ => { }
        };

        examples.push(vector{ data: example });
    }

    let mut rng = thread_rng();
    let mut means: Vec<vector> = Vec::new();

    for _ in range(0, k) {
        means.push(match rng.choose(examples.as_slice()) {
            Some(x) => x.clone(),
            None => panic!("Can't choose random element")
        });
    }

    let mut clusters = Vec::<usize>::new();
    clusters.resize(m, 0);

    let eps = 0.01;

    set_current_nearest(clusters.as_mut_slice(), examples.as_slice(), means.as_slice());
    update_means(clusters.as_slice(), examples.as_slice(), means.as_mut_slice());

    let mut cur = cost_function(clusters.as_slice(), examples.as_slice(), means.as_slice());
    let mut prev = cur;

    loop {

        set_current_nearest(clusters.as_mut_slice(), examples.as_slice(), means.as_slice());
        update_means(clusters.as_slice(), examples.as_slice(), means.as_mut_slice());

        cur = cost_function(clusters.as_slice(), examples.as_slice(), means.as_slice());

        println!("Log: cur = {}", cur);

        if (prev - cur).abs() < eps {
            break;
        }
        
        prev = cur;
    }

    println!("Accomplished");

    for i in range(0, k) {
        println!("Cluster number {}", i);
        for j in range(0, m) {
            if clusters[j] == i {
                println!("{:?}", examples[j]);
            }
        }
    }
}
