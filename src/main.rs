use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;

fn cidr_to_range(cidr: &str) -> Option<(Ipv4Addr, Ipv4Addr, Ipv4Addr)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let ip = Ipv4Addr::from_str(parts[0]).ok()?;
    let prefix: u32 = parts[1].parse().ok()?;
    if prefix > 32 {
        return None;
    }

    let ip_u32 = u32::from(ip);
    let mask = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };
    let network = ip_u32 & mask;
    let broadcast = network | !mask;

    let start = network;
    let end = broadcast;

    Some((
        Ipv4Addr::from(start),
        Ipv4Addr::from(end),
        Ipv4Addr::from(broadcast),
    ))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ip/cidr>", args[0]);
        return;
    }

    let input = &args[1];

    match cidr_to_range(input) {
        Some((start, end, broadcast)) => {
            println!("Start: {}", start);
            println!("End: {}", end);
            println!("Broadcast: {}", broadcast);
        }
        None => println!("Invalid CIDR format"),
    }
}
