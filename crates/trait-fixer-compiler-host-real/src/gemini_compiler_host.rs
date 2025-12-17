macro_wrapper_lib::MODULE_HEADER!();

// Use statements
macro_wrapper_lib::wrap_use! {
    use rustc_driver;
    use rustc_interface;
    use rustc_errors;
    use gemini_rustc_data_structures::{get_diagnostics, clear_diagnostics};
    use std::io::Write;
    use serde_json;
}

// Struct for collecting diagnostics
macro_wrapper_lib::wrap_struct! {
    struct DiagnosticCollectingCallbacks;
}

// Implement rustc_driver::Callbacks for DiagnosticCollectingCallbacks
macro_wrapper_lib::wrap_impl! {
    impl rustc_driver::Callbacks for DiagnosticCollectingCallbacks {
        macro_wrapper_lib::wrap_fn! {
            fn after_analysis<'tcx>(
                &mut self,
                _compiler: &rustc_interface::interface::Compiler,
            ) -> rustc_driver::Compilation {
                let diagnostics = get_diagnostics();
                let json_output = serde_json::to_string_pretty(&diagnostics)
                    .expect("Failed to serialize diagnostics to JSON");
                std::io::stdout().write_all(json_output.as_bytes())
                    .expect("Failed to write diagnostics JSON to stdout");

                rustc_driver::Compilation::Continue
            }
        }

        // Other required methods of rustc_driver::Callbacks can be added here as needed,
        // potentially forwarding to a wrapped original callback or being no-ops.
        // For this task, `after_analysis` is the primary one.
    }
}

// Function to run the compiler with the Gemini emitter
macro_wrapper_lib::wrap_fn! {
    pub fn run_compiler_with_gemini_emitter(args: Vec<String>) {
        clear_diagnostics();

        let custom_emitter = Box::new(rustc_errors::CollectedEmitter::new(
            rustc_errors::translation::Translator::new("en-US".to_string()),
            None,
        ));

        let config = rustc_interface::Config {
            opts: rustc_driver::args::arg_slice_to_options(&args).unwrap(), // Convert args to Options
            crate_cfg: Vec::new(),
            crate_disambiguator: "gemini_trait_fixer_diags".to_string(),
            input: None,
            output_dir: None,
            output_file: None,
            file_loader: None,
            lint_caps: Default::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            ice_file: None,
            using_internal_features: &std::sync::atomic::AtomicBool::new(false),
            custom_emitter: Some(custom_emitter),
        };

        let mut our_callbacks = DiagnosticCollectingCallbacks;

        // The third argument is `Box<dyn Callbacks + Send>`, so we use `&mut our_callbacks`
        rustc_driver::run_compiler(args, &mut our_callbacks, Some(config));
    }
}

macro_wrapper_lib::MODULE_FOOTER!();
