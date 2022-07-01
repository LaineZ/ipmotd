use rand::{thread_rng, Rng};

use crate::pearls::{get_pages_in_category, ip_get};

mod pearls;

fn main() -> anyhow::Result<()> {
    let mut rng = thread_rng();
    let args: Vec<String> = std::env::args().collect();
    let empty_str = String::new();
    let category = args.get(1).unwrap_or(&empty_str);
    let pages = get_pages_in_category(category)?;

    let page = rng.gen_range(0..pages);
    let quotes = ip_get(category, page)?;
    
    println!("{}", quotes[rng.gen_range(0..quotes.len() - 1)]);
    Ok(())
}
