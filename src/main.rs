use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-config")]
struct Opt {
    #[structopt(name="names")]
    names: Vec<String>,

    #[structopt(long)]
    cflags: bool,

    #[structopt(long)]
    libs: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.cflags {
        print!("-I\"..\\vcpkg\\installed\\x86-windows\\include\"");
    } else if opt.libs {
        print!("-L\"..\\vcpkg\\installed\\x86-windows\\lib\"");
    }
}
