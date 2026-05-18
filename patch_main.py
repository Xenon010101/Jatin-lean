import sys
import re

with open("src/main.rs", "r") as f:
    content = f.read()

# Find fn handle_subcommand(command: Commands) -> Result<()> {
start_idx = content.find("fn handle_subcommand(command: Commands) -> Result<()> {")
if start_idx == -1:
    print("Could not find handle_subcommand")
    sys.exit(1)

# Find fn run_local_mode
end_idx = content.find("fn run_local_mode", start_idx)
if end_idx == -1:
    print("Could not find run_local_mode")
    sys.exit(1)

# We want to replace from start_idx up to the end of the previous function.
# But let's just replace from start_idx to the exact line before fn run_local_mode.
# Specifically, we know the previous function was ctrlc_handler.

end_idx = content.rfind("\n\n/// Run in local mode", start_idx, end_idx)
if end_idx == -1:
    # If not found, look for something else
    pass

new_func = """fn handle_subcommand(command: Commands, ctx: &output::OutputContext) -> Result<()> {
    match command {
        Commands::Node { command } => cli::handle_node_command(command, ctx)?,
        Commands::System { command } => cli::handle_system_command(command, ctx)?,
        Commands::Network { command } => cli::handle_network_command(command, ctx)?,
        Commands::Memory { command } => cli::handle_memory_command(command, ctx)?,
        Commands::Bench { command } => cli::handle_bench_command(command, ctx)?,
        Commands::Analyze { command } => cli::handle_analyze_command(command, ctx)?,
        Commands::Legacy(command) => cli::handle_legacy_command(command, ctx)?,
        Commands::AiContext { path } => ai_context::handle_ai_context(path, ctx)?,
    }
    Ok(())
}

/// Wrapper for the CLI node scan command to call the internal runner
pub fn run_local_mode_from_cli(
    path: &std::path::PathBuf,
    force: bool,
    yes: bool,
    verbose: bool,
    profile: bool,
    snapshot: bool,
    export: Option<&std::path::Path>,
    _ctx: &output::OutputContext,
) -> Result<()> {
    let target = std::fs::canonicalize(path)?;
    run_local_mode(&target, force, yes, verbose, None, profile, snapshot, export)
}"""

# Find the start of run_local_mode precisely
run_local_start = content.find("fn run_local_mode(")

# Find where handle_subcommand starts, and where ctrlc_handler ends.
ctrlc_end = content.find("}\n\n/// Run in local mode", start_idx)
if ctrlc_end != -1:
    end_replace = ctrlc_end + 1
else:
    # Find the last closing brace before run_local_start
    end_replace = content.rfind("}", start_idx, run_local_start) + 1

new_content = content[:start_idx] + new_func + content[end_replace:]

with open("src/main.rs", "w") as f:
    f.write(new_content)

print("Replaced handle_subcommand successfully")
