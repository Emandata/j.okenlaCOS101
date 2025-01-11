struct Laptop {
    quantity: u32,
    price_dell: u32,
    price_toshiba: u32,
    price_ibm: u32,
    price_hp: u32,
}

impl Laptop {
    fn total_dell(&self) -> u32 {
        self.price_dell * self.quantity
    }

    fn total_hp(&self) -> u32 {
        self.price_hp * self.quantity
    }

    fn total_toshiba(&self) -> u32 {
        self.price_toshiba * self.quantity
    }

    fn total_ibm(&self) -> u32 {
        self.price_ibm * self.quantity
    }
}

fn main() {
    let laptop = Laptop {
        quantity: 3,
        price_dell: 850_000,
        price_toshiba: 550_000,
        price_ibm: 755_000,
        price_hp: 650_000,
    };

    println!(
        "Total for HP Laptop is: {}\nTotal for Dell Laptop is: {}\nTotal for Toshiba Laptop is: {}\nTotal for IBM Laptop is: {}",
        laptop.total_hp(),
        laptop.total_dell(),
        laptop.total_toshiba(),
        laptop.total_ibm()
    );
}
