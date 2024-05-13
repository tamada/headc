#[derive(Debug,Parser)] 

  pub struct CliOpts {
    #[arg(short = 'n',long default_value = 10)] 
    lines: Option<u8>,
    #[arg(short = 'c',long)] 
    bytes: String,
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
    }
