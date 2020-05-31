extern crate treebitmap;
use std::net::Ipv4Addr;
use std::time::Instant;
use treebitmap::IpLookupTable;

fn time_insertion(tree: &mut IpLookupTable<Ipv4Addr, String>) {
    let start = Instant::now();
    let mut count = 0;

    for o1 in 0..128 {
        for o2 in 0..128 {
            for o3 in 0..128 {
                tree.insert(Ipv4Addr::new(o1, o2, o3, 0), 24, "US".into());
                count += 1;
            }
        }
    }

    for o1 in 128..=255 {
        for o2 in 128..=255 {
            for o3 in 128..=255 {
                tree.insert(Ipv4Addr::new(o1, o2, o3, 0), 16, "CA".into());
                count += 1;
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "inserted {} entries in {} milliseconds",
        count,
        duration.as_millis()
    );

    let rate = ((count as f64) / (duration.as_millis() as f64)) * 1000.;
    println!("rate: {} entries per second", rate);
}

fn time_lookup(tree: &IpLookupTable<Ipv4Addr, String>) {
    let start = Instant::now();
    let mut count: u64 = 0;

    for o1 in 0..=255 {
        for o2 in 0..=255 {
            for o3 in 0..=255 {
                let lookup_ip = Ipv4Addr::new(o1, o2, o3, 1);
                let _result = tree.longest_match(lookup_ip);
                count += 1;
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "looked up {} ips in {} milliseconds",
        count,
        duration.as_millis()
    );

    let rate = ((count as f64) / (duration.as_millis() as f64)) * 1000.;
    println!("rate: {} lookups per second", rate);
}

pub fn main() {
    let mut tree = IpLookupTable::new();
    println!("Insertion stats");
    time_insertion(&mut tree);
    println!();
    println!("Lookup stats");
    time_lookup(&tree);

    println!();
    println!("Random Examples");
    tree.insert(Ipv4Addr::new(255, 255, 255, 255), 32, "N/A".into());

    let lookup_ip = Ipv4Addr::new(1, 1, 1, 1);
    let result = tree.longest_match(lookup_ip);
    println!("ip: {}, result: {:?}", lookup_ip, result);

    let lookup_ip = Ipv4Addr::new(129, 129, 1, 1);
    let result = tree.longest_match(lookup_ip);
    println!("ip: {}, result: {:?}", lookup_ip, result);

    let lookup_ip = Ipv4Addr::new(255, 255, 255, 255);
    let result = tree.longest_match(lookup_ip);
    println!("ip: {}, result: {:?}", lookup_ip, result);
}
