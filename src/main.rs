mod config;

fn main() {
    let cfg = match config::get_config() {
        Ok(cfg) => cfg,
        Err(_) => panic!("Couldn't get config"),
    };

    for e in cfg.entries {
        for tc in e.termination_codes {
            println!("{} {:?}",e.name, tc)
        }
    }
}
