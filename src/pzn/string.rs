#[cfg(test)]
mod string_test {

    #[test]
    fn string_slice() {

        let name = "    Andi Hasan A    ";
        let trim = name.trim();

        println!("{}", name);
        println!("{}", trim);
    }

    #[test]
    fn string() {

        let mut name = String::from("Andi Hasan");
        println!("{}", name);
        name.push_str(" Ashari");
        println!("{}", name);

        let budi = name.replace("Andi", "Budi");
        println!("{}", name);
        println!("{}", budi);
    }
}