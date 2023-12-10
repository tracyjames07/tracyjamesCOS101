use std::io::Write;
use std::fs::OpenOptions;

fn main() {

    let mut index = vec!["\n\nINDEX", "\n  1", "\n  2", "\n  3", "\n  4", "\n  5"];
    let mut commisioner_name = vec!["\t\tCOMMISIONER NAME", "\t\t\tAigbogun Alamba Daudu", "\t\t\tMurtala Afeez Bendu", "\t\t\tOkorocha Calistus Ogbonna", "\t\t\tAdewale Jimoh Akanbi", "\t\t\tOsazuwa Faith Etieye"];
    let mut ministry = vec!["\t\t\tMINISTRY", "\t\tInternal Affairs", "\t\t\tJustice", "\tDefense", "\t\tPower & Steel", "\t\tPetroleum"];
    let mut geo_zone = vec!["\t\t\tGEOPOLITICAL ZONE", "\tSouth West", "\t\t\t\tNorth East", "\t\t\t\tSouth South", "\t\tSouth West", "\t\t\tSouth East"];

    let mut isd = std::fs::File::create("EFCC-ISD.txt").expect("Unable to create file.");

    isd.write_all("Information Service Department".as_bytes()).expect("Unable to write into the file.");

    for d in 0..index.len() {


        isd.write_all(index[d].as_bytes()).expect("Unable to write into the file.");

        isd.write_all(commisioner_name[d].as_bytes()).expect("Unable to write into the file.");

        isd.write_all(ministry[d].as_bytes()).expect("Unable to write into the file.");

        isd.write_all(geo_zone[d].as_bytes()).expect("Unable to write into the file.");

    }
}