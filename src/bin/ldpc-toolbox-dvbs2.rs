use ldpc_toolbox::codes::dvbs2::Code;
use structopt::StructOpt;

type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ldpc-toolbox-dvbs2",
    about = "Generates the alist of DVB-S2 LDPCs"
)]
struct Opt {
    /// Coding rate
    #[structopt(short, long)]
    rate: String,

    /// Enables short FECFRAME
    #[structopt(long)]
    short: bool,
}

impl Opt {
    fn code(&self) -> Result<Code> {
        match (&*self.rate, self.short) {
            ("1/4", false) => Ok(Code::R1_4),
            ("1/3", false) => Ok(Code::R1_3),
            ("2/5", false) => Ok(Code::R2_5),
            ("1/2", false) => Ok(Code::R1_2),
            ("3/5", false) => Ok(Code::R3_5),
            ("2/3", false) => Ok(Code::R2_3),
            ("3/4", false) => Ok(Code::R3_4),
            ("4/5", false) => Ok(Code::R4_5),
            ("5/6", false) => Ok(Code::R5_6),
            ("8/9", false) => Ok(Code::R8_9),
            ("9/10", false) => Ok(Code::R9_10),
            ("1/4", true) => Ok(Code::R1_4short),
            ("1/3", true) => Ok(Code::R1_3short),
            ("2/5", true) => Ok(Code::R2_5short),
            ("1/2", true) => Ok(Code::R1_2short),
            ("3/5", true) => Ok(Code::R3_5short),
            ("2/3", true) => Ok(Code::R2_3short),
            ("3/4", true) => Ok(Code::R3_4short),
            ("4/5", true) => Ok(Code::R4_5short),
            ("5/6", true) => Ok(Code::R5_6short),
            ("8/9", true) => Ok(Code::R8_9short),
            _ => Err(self.code_error()),
        }
    }

    fn code_error(&self) -> String {
        let fecframe = if self.short { "short" } else { "normal" };
        format!("Invalid rate {} for {} FECFRAME", self.rate, fecframe)
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    print!("{}", opt.code()?.h().alist());
    Ok(())
}
