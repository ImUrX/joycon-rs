#![allow(unused_must_use)]

use joycon_rs::prelude::*;
use joycon_rs::joycon::{JoyConManager, SimpleJoyConDriver};
use joycon_rs::joycon::lights::*;

fn main() -> JoyConResult<()> {
    let (tx, rx) =
        std::sync::mpsc::channel();

    let _output = std::thread::spawn(move || {
        // Push buttons or tilt the stick please.
        // Stop with `Cmd + C` or `Ctrl + C`
        while let Ok(message) = rx.recv() {
            dbg!(message);
        }
    });

    let manager = JoyConManager::get_instance();
    let (managed_devices, new_devices) = {
        let lock = manager.lock();
        match lock {
            Ok(m) => (m.managed_devices(),m.new_devices()),
            Err(_) => unreachable!()
        }
    };

    managed_devices.into_iter()
        .chain(new_devices)
        .try_for_each::<_, JoyConResult<()>>(|d| {
            let mut driver = SimpleJoyConDriver::new(&d)?;
            let tx = tx.clone();

            std::thread::spawn(move || {
                driver.set_player_lights(&vec![], &vec![]).unwrap();
                driver.set_player_lights(&vec![LightUp::LED1], &vec![Flash::LED1]).unwrap();
                tx.send(driver.get_player_lights()).unwrap();
            });

            Ok(())
        })?;

    Ok(())
}