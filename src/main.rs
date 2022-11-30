use std::io::Write;

use clap::Parser;

mod markov;
use markov::Model;

#[derive(clap::Subcommand, Debug)]
enum Action {
    Train {
        source: std::path::PathBuf,
        model_file: std::path::PathBuf,

        #[arg(short, long, default_value_t = false)]
        reset: bool,
    },
    Generate {
        model_file: std::path::PathBuf,
        out_file: std::path::PathBuf,

        #[arg(short, long, default_value_t = 10)]
        count: u32,
    },
}

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Train {
            source,
            model_file: model_file_path,
            reset,
        } => {
            // load source file
            let source_file =
                std::fs::File::open(source).expect("Unexpected error opening source file");
            let mut source_reader = std::io::BufReader::new(source_file);

            // load model file
            let mut model = {
                let prev_model = std::fs::OpenOptions::new()
                    .read(true)
                    .open(&model_file_path);

                let model_exists = match prev_model {
                    Ok(_) => true,
                    Err(_) => false,
                };

                // make new model if file didn't exist or we're resetting
                if !model_exists || reset {
                    Model::new_prose()
                } else {
                    // TODO: should i borrow prev_model??
                    serde_json::from_reader(&prev_model.unwrap()).expect("Invalid model file")
                }
            };

            // train model
            println!("Training...");
            model.train_buf(&mut source_reader);
            println!("Training complete!");

            // serialize model
            let serialized = serde_json::to_string(&model)?;

            // open up model file and clear it
            let mut model_file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&model_file_path)
                .expect("Could not open model file for writing");

            // save serialized model
            model_file.write_all(serialized.as_bytes())?;
            model_file.flush().expect("Error finalizing writing");
            println!("Saved model to {}.", model_file_path.to_str().unwrap());

            Ok(())
        }
        Action::Generate {
            model_file,
            out_file,
            count,
        } => {
            let source_reader =
                std::fs::read_to_string(model_file).expect("Unable to read model file");
            let model: Model =
                serde_json::from_str(&source_reader[..]).expect("Invalid model file");

            let mut out_file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(out_file)
                .expect("Unable to open out file");

            for _ in 0..count {
                out_file
                    .write_all(format!("{}\n", model.generate_paragraph()).as_bytes())
                    .expect("Error writing to file");
            }

            out_file.flush().expect("Error finalizing writing");

            Ok(())
        }
    }
}
