use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::default()
        .with_style("fluent".into());

    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}