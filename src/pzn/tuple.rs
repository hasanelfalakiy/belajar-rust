#[cfg(test)]
mod tuple_tests {
    /*
    Tuple = tipe data kumpulan lebih dari satu tipe data,
    - jumlah data di tuple sudah final artinya tidak bisa berkurang atau bertambah,
    - jika kita membuat tuple dg total 3 data, maka tidak akan bisa diubah lagi jumlah data dan juga tipe datanya,
    - untuk membuat tuple kita bisa gunakan () tanda kurung
     */
    #[test]
    fn tuple() {

        // tipe data nya boleh ditulis atau tidak, jika ditulis :(i32, f64, bool)
        let data = (10, 10.5, true);

        // tanda :? adalah debug information
        println!("{:?}", data);

        // cara mengkases data tuple menggunakan titik dikuti nomor indeks nya, nomor indeks tuple dimulai dari 0
        /*let a = data.0;
        let b = data.1;
        let c = data.2;
        println!("{} {} {}", a, b, c);*/

        /*
        Destructuring Tuple = membongkar isi data tuple dengan tujuan untuk menyimpannya ke variable secara langsung
        alih-alih dengan mengkasesnya satu per satu, jika ada data di tuple yang tidak kita butuhkan, kita bisa gunakan tanda _ (underscore)
        saat melakukan destructuring tuple
         */
        let (a, b, _) = data;
        println!("{} {}", a, b);

    }

    /*
    Mutable Tuple
    saat kita buat immutable variable, secara otomatis tuple akan mengikuti tidak bisa diubah lagi isi datanya,
    namun, jika kita membuat variable dengan bentuk mutable, maka data tuple bisa kita ubah
    untuk mengubah data tuple, kita cukup gunakan nomor indeks dan = (tanda sama dengan)
    mirip seperti mengubah data variable biasanya.
     */
    #[test]
    fn mutable_tuple() {
        let mut data = (10, 10.5, true);
        data.0 += 10;
        data.1 = 20.5;
        data.2 = false;
        println!("{:?}", data); // hasilnya (20, 20.5, false)
    }

    /*
    Unit
    unit adalah tuple tanpa nilai apapun, ditulisnya ()
    biasanya unit ini digunakan untuk method/function yang tidak membutuhkan hasil data apapun
     */
    fn unit() {
        println!("Hai");
    }

    #[test]
    fn test_unit() {
        let result = unit();
        println!("{:?}", result);

        let test = ();
        println!("{:?}", test);
    }
}