use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use rand::Rng;

use crate::{
    model::PearlCategory,
    pearls::{get_pages_in_category, ip_get},
};

mod model;
mod pearls;

fn write_db(model: &PearlCategory, path: PathBuf) -> anyhow::Result<()> {
    let mut file = fs::File::create(path)?;
    let database = serde_json::to_string(&model)?;
    file.write_all(database.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn read_db(path: PathBuf) -> anyhow::Result<PearlCategory> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let database: PearlCategory = serde_json::from_str(&buf)?;
    Ok(database)
}

fn init_db(category: &String, path: PathBuf) -> anyhow::Result<PearlCategory> {
    let mut rng = rand::thread_rng();

    let mut model = model::PearlCategory {
        name: category.clone(),
        page_count: get_pages_in_category(category)?,
        pearls: Vec::new(),
    };

    let page = rng.gen_range(0..model.page_count);

    model.pearls.extend(ip_get(model.clone().name, page)?);
    write_db(&model, path)?;
    Ok(model)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let empty_str = String::new();
    let category = args.get(1).unwrap_or(&empty_str);
    let path = home::home_dir()
        .unwrap_or(PathBuf::from("."))
        .join("ip.db");

    if !path.exists() {
        init_db(category, path.clone())?;
    }

    let mut state = read_db(path.clone())?;

    if state.pearls.is_empty() || state.name != *category {
        state = init_db(category, path.clone())?;
    }

    println!("{}", state.pearls[0]);
    state.pearls.remove(0);

    write_db(&state, path)?;
    Ok(())
}
