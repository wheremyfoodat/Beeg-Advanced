#![warn(clippy::all)]
#![allow(nonstandard_style)]

pub mod gba;
pub mod bus;
pub mod cpu;
pub mod mem;
pub mod ppu;
pub mod ARM;
pub mod io;
pub mod Thumb;
pub mod barrelShifter;
pub mod helpers;
//mod frontend;

use gba::GBA;
//use imgui::*;

fn main() {
/*
    let system = frontend::init("Beeg Advanced");
    
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
*/
    let mut gba = GBA::new("ROMs/ARMWrestlerFixed.gba".to_string());
    gba.init();

    loop {
        gba.step();
    }
}