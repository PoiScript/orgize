// Author: Alex Roper <alex@aroper.net>

use rand::Rng;
use rand::RngCore;
use serde_json::to_writer;

use std::convert::TryInto;
use std::io::Read;

fn fuzz(
    stats: std::sync::mpsc::Sender<(
        usize,
        usize,
        usize,
        usize,
        std::time::Duration,
        std::time::Duration,
    )>,
) {
    let mut rng = rand::thread_rng();

    let mut s = String::default();
    let mut s2 = String::default();

    let mut counter: usize = 0;
    let mut errors: usize = 0;
    let mut jumbo: usize = 0;
    let mut bytes: usize = 0;
    let mut assembly_time = std::time::Duration::new(0, 0);
    let mut parse_time = std::time::Duration::new(0, 0);
    loop {
        if counter % 100 == 1 {
            stats
                .send((counter, jumbo, errors, bytes, assembly_time, parse_time))
                .unwrap();

            counter = 0;
            errors = 0;
            jumbo = 0;
            bytes = 0;
            assembly_time = std::time::Duration::new(0, 0);
            parse_time = std::time::Duration::new(0, 0);
        }

        let mut length = rng.next_u32() % 50;
        if length == 18 {
            length = rng.next_u32() % 500 + 15;
        }
        if length == 19 && rng.next_u32() % 7 == 3 {
            length = rng.next_u32() % 10000 + 10000;
            jumbo += 1;
        }

        let length = length as usize;

        let t = std::time::Instant::now();
        s.clear();

        let structure = "* -\n\r\t:@%<>-[]_#";
        let structure_len = structure.chars().count();

        while s.len() < length {
            let r = rng.gen_range(0, 7);
            if r == 0 {
                s += "* ";
            } else if r == 1 {
                s += "\n* ";
            } else if r == 2 {
                s.push('\n');
            } else if r == 3 {
                s.extend(
                    rng.sample_iter(rand::distributions::Alphanumeric)
                        .take((rng.next_u32() % 20).try_into().unwrap()),
                );
                s.push(' ')
            } else if r == 4 {
                s.extend(
                    rng.sample_iter::<char, _>(rand::distributions::Standard)
                        .take((rng.next_u32() % 20 as u32).try_into().unwrap()),
                );
                s.push(' ')
            } else if r == 5 {
                s.push(
                    structure
                        .chars()
                        .nth(rng.gen_range(0, structure_len))
                        .unwrap(),
                );
            } else {
                s.extend(
                    rng.sample_iter::<char, _>(rand::distributions::Standard)
                        .take((rng.next_u32() as usize % length).try_into().unwrap()),
                );
            }
        }
        assembly_time += t.elapsed();
        bytes += s.len();

        let t = std::time::Instant::now();
        let org = orgize::Org::parse(&s);

        if !org.validate().is_empty() {
            errors += 1;
        } else {
            let mut buf = iobuffer::IoBuffer::default();

            if org.write_org(&mut buf).is_err() {
                errors += 1;
            }

            buf.read_to_string(&mut s2).unwrap();
            if !orgize::Org::parse(&s2).validate().is_empty() {
                errors += 1;
            }
            s2.clear();

            if org.write_html(&mut buf).is_err() {
                errors += 1;
            }

            if to_writer(&mut buf, &org).is_err() {
                errors += 1;
            }
        }
        parse_time += t.elapsed();

        counter += 1;
    }
}

fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    let thread_count = if args.len() > 2 {
        println!("Usage: {} [num_threads]", args[0]);
        return;
    } else if args.len() == 1 {
        num_cpus::get()
    } else {
        str::parse(&args[1]).unwrap()
    };

    let (sender, receiver) = std::sync::mpsc::channel();

    let threads: Vec<_> = (0..thread_count)
        .map(|_| {
            let sender = sender.clone();
            std::thread::spawn(move || fuzz(sender))
        })
        .collect();

    let mut counter = 0;
    let mut jumbo = 0;
    let mut errors = 0;
    let mut bytes = 0;
    let mut assembly_time = std::time::Duration::new(0, 0);
    let mut parse_time = std::time::Duration::new(0, 0);

    let start = std::time::Instant::now();
    let mut last_update = std::time::Instant::now();
    let update_interval = std::time::Duration::from_secs(5);
    for (counter_delta, jumbo_delta, errors_delta, bytes_delta, assembly_delta, parse_delta) in
        receiver.iter()
    {
        counter += counter_delta;
        jumbo += jumbo_delta;
        errors += errors_delta;
        bytes += bytes_delta;
        assembly_time += assembly_delta;
        parse_time += parse_delta;

        if last_update.elapsed() > update_interval {
            last_update = std::time::Instant::now();
            println!("Running for: {:?}", start.elapsed());
            println!("Threads: {}", thread_count);
            println!("Strings tested: {}", counter);
            println!("Jumbo strings: {}", jumbo);
            println!("Errors: {}", errors);
            println!(
                "Total input: {}",
                bytesize::ByteSize::b(bytes.try_into().unwrap())
            );
            println!("Total generation time: {:?}", assembly_time);
            println!("Total parse time: {:?}", parse_time);
            println!("\n\n\n\n\n");
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
