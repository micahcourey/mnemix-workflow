fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        None => mnemix_workflow::run_ui_shortcut(),
        Some("--help") | Some("-h") => {
            println!("mnx launches the interactive Mnemix Workflow TUI.\n");
            println!("Usage:\n  mnx");
            std::process::ExitCode::SUCCESS
        }
        Some(argument) => {
            eprintln!("Unsupported argument for mnx: {argument}");
            eprintln!("Usage:\n  mnx");
            std::process::ExitCode::FAILURE
        }
    }
}
