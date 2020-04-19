use crate::{gui::Gui, input::Input, money::Wallet, phase::Phase};
use specs_blit::{specs::*, PixelBuffer};

pub const HOLD_PRICE: usize = 2000;
pub const SPLIT_PRICE: usize = 1000;

#[derive(Debug, Default)]
pub struct Upgrades {
    hold: bool,
    split: bool,
}

impl Upgrades {
    pub fn render(
        &mut self,
        buffer: &mut PixelBuffer,
        gui: &mut Gui,
        wallet: &mut Wallet,
        phase: &mut Phase,
        input: &Input,
        level: usize,
    ) {
        let (x, y) = Upgrades::buttons()[0].0;

        gui.draw_label(buffer, "Click to buy upgrade.", x, y - 50);
        gui.draw_label(
            buffer,
            format!("You have {} scrap.", wallet.money()),
            x,
            y - 30,
        );

        let pos = Upgrades::buttons()[0].0;
        if !self.hold {
            gui.draw_label(
                buffer,
                format!(
                    "Hold the ball & release it on click\n({} scrap)",
                    HOLD_PRICE
                ),
                pos.0 + 10,
                pos.1 + 5,
            );
            if Upgrades::pressed(input, 0) && wallet.money() >= HOLD_PRICE {
                self.hold = true;
                wallet.subtract(HOLD_PRICE);
            }
        } else {
            gui.draw_label(buffer, "Already bought", pos.0 + 10, pos.1 + 5);
        }

        let pos = Upgrades::buttons()[1].0;
        if !self.split {
            gui.draw_label(
                buffer,
                format!("Big balls split on contact\n({} scrap)", SPLIT_PRICE),
                pos.0 + 10,
                pos.1 + 5,
            );
            if Upgrades::pressed(input, 1) && wallet.money() >= SPLIT_PRICE {
                self.split = true;
                wallet.subtract(SPLIT_PRICE);
            }
        } else {
            gui.draw_label(buffer, "Already bought", pos.0 + 10, pos.1 + 5);
        }

        let pos = Upgrades::buttons()[2].0;
        gui.draw_label(
            buffer,
            format!("Start level {}", level),
            pos.0 + 10,
            pos.1 + 5,
        );

        if Upgrades::pressed(input, 2) {
            *phase = Phase::SwitchTo(Box::new(Phase::Play));
        }
    }

    pub fn pressed(input: &Input, index: usize) -> bool {
        let ((x, y), (w, h)) = Upgrades::buttons()[index];
        let (mx, my) = (input.mouse_x(), input.mouse_y());

        input.mouse_down() && mx >= x && mx < x + w && my >= y && my < y + h
    }

    pub fn buttons() -> Vec<((i32, i32), (i32, i32))> {
        let x = 25;
        let y = 100;
        vec![
            ((x, y), (crate::WIDTH as i32 - x * 2, 30)),
            ((x, y + 40), (crate::WIDTH as i32 - x * 2, 30)),
            ((x, y + 120), (crate::WIDTH as i32 - x * 2, 20)),
        ]
    }
}
