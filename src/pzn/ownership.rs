#[cfg(test)]
mod ownership_tests {

    #[test]
    fn ownership_rules() {

        // a tidak bisa diakses di atas sini, belum dideklarasikan
        let a = 10; // a bisa diakses mulai disini

        {// b tidak bisa diakses di atas sini, belum dideklarasikan
            let b = 20; // b bisa diakses mulai disini
            println!("{}", b);
        } // scope b selesai, b dihapus, b tidak bisa diakses lagi
        println!("{}", a);
        // println!("{}", b); // error b tidak bisa diakses, karena diluar scope dan b sudah dihapus
    } // scope a selesai, a dihapus, a tidak bisa diakses lagi
}