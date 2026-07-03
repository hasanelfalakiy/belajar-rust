#[cfg(test)]
mod boolean_operator_test {
    #[test]
    fn boolean_operator() {
        let absen = 75;
        let nilai_akhir = 80;

        let lulus_absen: bool = absen >= 75;
        let lulus_nilai_akhir: bool = nilai_akhir >= 75;

        let lulus: bool = lulus_absen && lulus_nilai_akhir;
        println!("{}", lulus);
    }
}