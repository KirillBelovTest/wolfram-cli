use std::str::FromStr;

use clap::Parser;

use wolfram_app_discovery::WolframApp;
use wolfram_expr::{Expr, Symbol};
use wstp::kernel::WolframKernelProcess;

//==========================================================
// CLI Argument Declarations
//==========================================================

#[derive(Debug)]
#[derive(clap::Parser)]
#[command(name = "wolfram-cli", author, version, about)]

struct Cli {
	#[arg(short, long, action = clap::ArgAction::Count)]
	verbosity: u8,

	#[command(subcommand)]
	command: Command,
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
enum Command {
	#[command(subcommand)]
	Paclet(PacletCommand),
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
enum PacletCommand {
	New {
		name: String,
		#[arg(
			long = "base",
			short = 'b',
			help = "use paclet base name as directory name"
		)]
		shorten_to_base_name: bool,
	},
}

//==========================================================
// main()
//==========================================================

fn main() {
	let args = Cli::parse();

	// dbg!(&args);

	let Cli { verbosity, command } = args;

	match command {
		Command::Paclet(paclet_command) => handle_paclet_command(paclet_command, verbosity),
	}
}

fn handle_paclet_command(command: PacletCommand, verbosity: u8) {
	match command {
		PacletCommand::New {
			shorten_to_base_name,
			name,
		} => handle_paclet_new(name, shorten_to_base_name, verbosity),
	}
}

//==========================================================
// $ wolfram paclet ...
//==========================================================

fn handle_paclet_new(name: String, shorten_to_base_name: bool, verbosity: u8) {
	let paclet_parent_dir =
		std::env::current_dir().expect("unable to get current working directory");

	// if verbosity > 0 {
	// 	eprintln!(
	// 		"creating paclet with name: {name} at {}",
	// 		paclet_root.display()
	// 	)
	// }

	//----------------------------------------------
	// Find a suitable Wolfram Language installation
	//----------------------------------------------

	let app = WolframApp::try_default().expect("unable to find any Wolfram Language installations");

	let wolfram_version = app.wolfram_version().unwrap();

	if wolfram_version.major() < 13
		|| (wolfram_version.major() == 13 && wolfram_version.minor() < 1)
	{
		panic!("incompatible Wolfram version: {wolfram_version}. 13.1 or newer is required for this command.")
	}

	//------------------------------------------------------
	// Launch the WolframKernel to evaluate CreatePaclet[..]
	//------------------------------------------------------

	let mut kernel = {
		let exe = app.kernel_executable_path().unwrap();

		WolframKernelProcess::launch(&exe).expect("unable to launch WolframKernel")
	};

	// Evaluate:
	//
	//     Needs["PacletTools`"]
	kernel
		.link()
		.put_eval_packet(&Expr::normal(
			Symbol::new("System`Needs"),
			vec![Expr::string("PacletTools`")],
		))
		.expect(r#"error evaluating Needs["PacletTools`"]"#);

	// Evaluate:
	//
	//     CreatePaclet[name, paclet_root]
	kernel
		.link()
		.put_eval_packet(&Expr::normal(
			Symbol::new("PacletTools`CreatePaclet"),
			vec![
				Expr::string(&name),
				Expr::string(
					paclet_parent_dir
						.to_str()
						.expect("paclet parent directory is not valid UTF-8"),
				),
			],
		))
		.expect(r#"error evaluating CreatePaclet[..]"#);

	// Evaluate:
	//
	//     Exit[]
	kernel
		.link()
		.put_eval_packet(&Expr::normal(Symbol::new("System`Exit"), vec![]))
		.expect(r#"error evaluating Exit[]"#);

	while let Ok(_) = kernel.link().get_token() {
		// Wait for the kernel to execute the commands we sent and shutdown
		// gracefully.
	}

	// TODO(cleanup): Change CreatePaclet to support an option for creating the
	//                new paclet with the base name directly, so we don't have
	//                to do this rename after it has been created.
	if let PacletName::Resource { publisher, base } =
		PacletName::from_str(&name).expect("malformed paclet name")
	{
		if shorten_to_base_name {
			// Use a double underscore instead of a '/' in the paclet root
			// directory name.
			let current = format!("{publisher}__{base}");
			let desired = base;

			std::fs::rename(
				paclet_parent_dir.join(&current),
				paclet_parent_dir.join(&desired),
			)
			.expect("error shortening paclet name")
		}
	};
}

enum PacletName {
	Resource { publisher: String, base: String },
	Normal(String),
}

impl FromStr for PacletName {
	type Err = String;

	fn from_str(name: &str) -> Result<Self, Self::Err> {
		let components: Vec<&str> = name.split('/').collect();

		match *components {
			[_] => Ok(PacletName::Normal(name.to_owned())),
			[publisher, base] => Ok(PacletName::Resource {
				publisher: publisher.to_owned(),
				base: base.to_owned(),
			}),
			[..] => Err(format!(
				"paclet names can contain at most one forward slash ('/') character: {:?}",
				name
			)),
		}
	}
}
