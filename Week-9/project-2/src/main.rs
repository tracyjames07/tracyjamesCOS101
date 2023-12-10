use std::io::Write;
use std::fs::OpenOptions;

fn main() {

    let mut student_name = vec!["\n\n\tNAME", "\nOluchi Mordi", "\nAdams Aliyu", "\nShania Bolade", "\nAdekunle Gold", "\nBlanca Edemoh"];
    let mut matric_number = vec!["\t\t\t  MATRICULATION NO.", "\t\t\tACC10211111", "\t\t\t\tECO10110101", "\t\t\tCSC10328828", "\t\t\tEEE11020202", "\t\t\tMEE10202001"];
    let mut department = vec!["\t\tDEPARTMENT", "\t\t\tAccounting", "\t\t\tEconomics", "\t\t\tComputer Science", "\t\t\tElectrical Engineering", "\t\t\tMechanical Engineering"];
    let mut level = vec!["\t\t\t\t\tLEVEL", "\t\t\t\t\t 300", "\t\t\t\t\t 100", "\t\t\t 200", "\t\t 200", "\t\t 100"];

    let mut pau_smis = std::fs::File::create("PAU-SMIS.xlsx").expect("Unable to create file.");

    pau_smis.write_all("PAU SMIS".as_bytes()).expect("Unable to write into the file.");

    for p in 0..student_name.len() {


        pau_smis.write_all(student_name[p].as_bytes()).expect("Unable to write into the file.");

        pau_smis.write_all(matric_number[p].as_bytes()).expect("Unable to write into the file.");

        pau_smis.write_all(department[p].as_bytes()).expect("Unable to write into the file.");

        pau_smis.write_all(level[p].as_bytes()).expect("Unable to write into the file.");

    }
}