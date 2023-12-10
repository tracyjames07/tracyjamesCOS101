use std::io::Write;

fn main() {

	let mut lager = vec!["\nLAGER", "\n33 Export", "\nDesperados", "\nGoldberg", "\nGuilder", "\nHeineken", "\nStar"];
	let mut stout = vec!["\t\tSTOUT", "\tLegend", "\tTurbo King", "\tWilliams", "\t", "\t", "\t"];
	let mut non_alchoholic = vec!["\t\tNON-ALCHOHOLIC DRINKS", "\t\tMaltina", "\tAmstel Malta", "\tMalta Gold", "\tFayrouz", "\t", "\t"];

	let mut catalogue = std::fs::File::create("catalogue.txt").expect("Unable to create catalogue.");

	catalogue.write_all("NIGERIAN BREWERIES PLC. \nDrinks Portfolio. \n \n".as_bytes()).expect("Unable to write into the catalogue.");

	for i in 0..lager.len() {

		catalogue.write_all(lager[i].as_bytes()).expect("Unable to write into the catalogue.");

		catalogue.write_all(stout[i].as_bytes()).expect("Unable to write into the catalogue.");

		catalogue.write_all(non_alchoholic[i].as_bytes()).expect("Unable to write into the catalogue.");

	}

	/*catalogue.write_all("	NIGERIAN BREWERIES PLC.
						   Drinks Portfolio.
				".as_bytes()).expect("Write failed.");

	catalogue.write_all("LAGER".as_bytes()).expect("Write failed.");
	catalogue.write_all(lager).expect("Write failed.");

	catalogue.write_all("STOUT".as_bytes()).expect("Write failed.");
	catalogue.write_all(stout.as_bytes()).expect("Write failed.");

	catalogue.write_all("NON-ALCHOHOLIC".as_bytes()).expect("Write failed.");
	catalogue.write_all(non_alchoholic.as_bytes()).expect("Write failed.");
*/
	println!("Data written into file.");

}