// cara menulis unit test diluar file main.rs
// cara menjslankan test tertentu:
// cargo test variable::tests::multi_variable -- --nocapture

#[cfg(test)]
mod tests {

    // cara menjalankan fungsi ini tanpa perlu meneruskan ke main func
    // terminal$: cargo test nama_fungsi_test -- --nocapture
    #[test]
    fn variable() {
        let nama_variable = "predifined value ";
        println!("{}", nama_variable);

        let mut message_number = 1;
        let message1 = "hello";
        println!("message number {}: {}", message_number, message1);

        message_number = 2; // boleh diubah jika variable dideklarasikan sebagai mutable dengan keyword mut
        let message2 = "world"; // deklarasi variable inference, compiler otomatis mengetahui tipe datanya
        println!("message number {}: {}", message_number, message2);

        // argument parameter macro
        message_number = 3;
        let message3: i8 = 24; // deklarasi variable manivest, tipe data ditulis eksplisit
                               /* terlihat seperti terbalik tapi lihat argumen indeks yang
                               	*  dimasukkan ke parameter macro di set ke 1 dulu baru ke 0
                               	*/
        println!("message number {1}: {0}", message3, message_number);

        // deklarasi variable sebelum mendefinisikan nilainya
        let message_num: i32;
        message_num = 12; // tapi assigment hanya boleh satu kali untuk immutable variable baik cara predefined atau defined
        println!("messeage num :{}", message_num);
    }

    #[test]
    fn multi_variable() {
        // deklarasi banyak variable dalam 1 baris statement, inference type
        let (var1, var2) = (24, "hello");
        println!("var1: {}", var1);
        println!("var2: {}", var2);

        // manifest type
        let (var3, var4): (i8, i8) = (32, 12);
        println!("var3: {}", var3);
        println!("var4: {}", var4);

        // salah satunya di mutable
        let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 6);
        println!("var5: {}", var5);
        println!("var6: {}", var6);
        var6 = 24;
        println!("var6: {}", var6);
        println!("var7: {}", var7);

        // deklarasi variable dengan tipe data ditentukan dari value
        let data1 = 24i8;
        println!("data1: {}", data1);

        // boleh dipisah dengan _ underscore
        let data2 = 24_i8;
        println!("data2: {}", data2);
    }

    #[test]
    fn shadowing() {
        let x = 5;
        println!("x: {}", x);

        let x = x + 1;
        println!("x: {}", x);
    }

    #[test]
    fn var_underscore() {
        let _ = 11;
    }
}
