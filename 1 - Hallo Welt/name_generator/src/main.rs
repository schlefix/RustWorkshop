use clap::{arg, command, value_parser, ArgAction, Command};


fn main() {
    println!("Hello, world!");

    let matches = command!() // requires `cargo` feature
            .subcommand(
                Command::new("gen")
                    .about("generiert Namen")
                    .subcommand(
                        Command::new("humans")
                    )
                    .subcommand(
                        Command::new("dwarfs")
                    )
                    .subcommand(
                        Command::new("elfs")
                    )
                    .arg(arg!(-l --list "liste Rassen").action(ArgAction::SetTrue))
            )
            .get_matches();

        if let Some(matches) = matches.subcommand_matches("gen") {
                println!("Generiere:");
           if *matches.get_one::<bool>("list").expect("defaulted by clap") {
                    println!(
                                "\t| {: ^10} | {: <10} |", "Eingabe", "Bedeutung"
                   );
                   println!( "\t{:=^27}","");
                   println!(    "\t| {: ^10} | {: <10} |",   "humans", "Menschen" );
                   println!(    "\t| {: ^10} | {: <10} |",   "dwarfs", "Zwerge"  );
                   println!(    "\t| {: ^10} | {: <10} |",   "elves", "Elfen"  );
                    println!( "\t{:=^27}","");
           } else {

               if let Some(human_matches) = matches.subcommand_matches("humans"){
                    println!("\tMenschennamen");
               }else if let Some(dwarf_matches) = matches.subcommand_matches("dwarfs"){
                    println!("\tZwergennamen");
               }else if let Some(elf_matches) = matches.subcommand_matches("elfs"){
                    println!("\tElfenamen");
               }else{
                    println!("Zufälligen Namen")
               }
           }
        }else{
            println!("Bye!!");
        }
}
