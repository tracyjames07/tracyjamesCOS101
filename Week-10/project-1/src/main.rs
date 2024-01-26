struct Laptop {

    hp: u32,
    ibm: u32,
    toshiba: u32,
    dell: u32,

}

impl Laptop {

    fn prices(&self) -> u32 {

        self.hp + self.ibm + self.toshiba + self.dell

    }

}

fn main() {

    let laptop1 = Laptop {

        hp: 650000 * 3,
        ibm: 755000 * 3,
        toshiba: 550000 * 3,
        dell: 850000 * 3,

    };

println!("\nThe price for three HP laptops is {}.
            The price for three IBM laptops is {}.
            The price for three Toshiba laptops is {}.
            The price for three Dell laptops is {}.",
            laptop1.hp, laptop1.ibm, laptop1.toshiba, laptop1.dell);

}