use specs_blit::specs::*;

#[derive(Debug, Default)]
pub struct Wallet {
    amount: usize,
}

impl Wallet {
    pub fn money(&self) -> usize {
        self.amount
    }

    pub fn add(&mut self, money: &Money) {
        self.amount += money.amount();
    }

    pub fn subtract(&mut self, amount: usize) {
        self.amount -= amount;
    }

    pub fn reset(&mut self) {
        self.amount = 0;
    }
}

#[derive(Component, Debug, Default)]
pub struct Money {
    amount: usize,
}

impl Money {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}
