use rich_sdl2_net_rust::Net;
use rich_sdl2_rust::Sdl;

fn main() {
    let sdl = Sdl::new();
    let net = Net::new(&sdl);
    let local_addresses = net.local_addresses();
    println!("Found {} local addresses", local_addresses.len());
    for (i, addr) in local_addresses.into_iter().enumerate() {
        println!("{}: {}", i + 1, addr);
    }
}
