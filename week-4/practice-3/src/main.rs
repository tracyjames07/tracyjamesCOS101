fn main() {
   let name = "Ayomide Adesokan";
   println!("My name is {}", name);

   let name2 = name.replace("Ayomide", "Adebare");
   println!("You can also call me {}", name2);

   let faculty = "Faculty of Science and Technology";

   let school = faculty.replace("Faculty", "School");
   println!("I am a student of the {}", school);
}