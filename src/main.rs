use enigo::Direction;
use enigo::*;
use notify::event;
use notify::*;
use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;

fn reload_stella(asm: &String) {
    let stem = asm.split('.').next().unwrap(); // fuck it
    let formatted = format!("-o{}.bin", stem);
    let _ = Command::new("swaymsg")
        .args(&["[title=\"Stella 7.0\"]", "focus"])
        .output()
        .expect("Failed to focus Stella.");
    let _ = Command::new("dasm")
        .args(&[asm, "-f3", &formatted])
        .output()
        .expect("assembly failed!");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Control, Direction::Press).unwrap();
    enigo.key(Key::Unicode('r'), Direction::Click).unwrap();
    enigo.key(Key::Control, Direction::Release).unwrap();
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let watchfile = &args[1];
    let path = Path::new(watchfile);
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;
    for res in rx {
        match res {
            Ok(Event { kind, .. }) => match kind {
                notify::EventKind::Modify(event::ModifyKind::Metadata(
                    event::MetadataKind::Any,
                )) => {
                    println!("modify!");
                    reload_stella(watchfile);
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
