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

    #[test]
    fn clone() {
        let name1 = String::from("Andi");
        println!("{}", name1);

        // kalau hanya let name2 = name1; saja maka pindah kepemilikan
        let name2 = name1.clone(); // tapi clone itu berat
        println!("{} {}", name1, name2);
    }
}