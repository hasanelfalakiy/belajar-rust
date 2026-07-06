#[cfg(test)]
mod ownership_movement_test {
    #[test]
    fn ownership_movement() {
        let name1 = String::from("Andi");
        println!("{}", name1);

        // ownership dari name1 dipindahkan ke name2
        let name2 = name1;
        // name1 tidak bisa diakses disini

        // println!("{}", name1); name1 tidak bisa diakses
        println!("{}", name2);
    }
}