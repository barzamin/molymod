use clap::Parser;
use molymod_dms::DMSFile;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    dms_path: PathBuf,
}

/*
   1) Fri May 27 15:06:31 2016 12028:gullingj:Justin Gullingsrud,nystaff
     version: msys/1.7.140
     workdir: /u/nyc/gullingj/git/gerrit/sw/libs/msys/.
     cmdline: /proj/desres/root/CentOS6/x86_64/msys/1.7.140/bin/dms-select tests/files/2f4k.dms --structure-only -o 2f4k.dms
  executable: /proj/desrad-c/root/Linux/x86_64/Python/2.7.11-03st/bin/python2.7 */


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let dms = DMSFile::open(args.dms_path)?;
    println!("dms ver: {:?}", dms.get_ver()?);
    println!("provenance:");
    for prov in dms.provenance()? {
        println!("{}) {} {}\n   version: {}\n   workdir: {}\n   cmdline: {}\nexecutable: {}", prov.id, prov.timestamp, prov.user, prov.version, prov.workdir, prov.cmdline, prov.executable);
    }

    Ok(())
}
