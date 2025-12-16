use std::fs;
use std::io::{self, BufRead};
use tera::{Context, Tera};
use serde::{Serialize, Deserialize};
use regex::Regex; // Add this line

const RAW_LOG_FILE: &str = "type_usage_raw.log";
const LABELED_LOG_FILE: &str = "type_usage_labeled.log";
const TEMPLATE_DIR: &str = "./templates/**/*";

const MARKDOWN_REPORT_TEMPLATE_NAME: &str = "type_system_report.tera";
const MARKDOWN_REPORT_OUTPUT_FILE: &str = "type_system_report.md";

const JSON_DATA_OUTPUT_FILE: &str = "type_usage_data.json";

const MACRO_REFACTOR_TEMPLATE_NAME: &str = "macro_refactor.tera";
const MACRO_REFACTOR_OUTPUT_FILE: &str = "generated_macro_refactors.rs";


#[derive(Serialize, Deserialize, Debug)]
struct TypeUsage {
    original_name: String,
    labeled_name: String,
    file_path: String,
    line_number: u32,
    context: String, // The actual line of code

    // New fields for extracted arguments
    lifetime_a: Option<String>,
    lifetime_b: Option<String>,
    drt_type: Option<String>,
    // For DeriveResolution in map(|DeriveResolution { path, item, exts: _, is_const }|
    destructuring_pattern: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse logs and create TypeUsage data
    let type_usages = parse_type_usage_logs()?;

    if type_usages.is_empty() {
        println!("No type usages found to report.");
        return Ok(());
    }

    // 2. Output structured data as JSON
    let json_output = serde_json::to_string_pretty(&type_usages)?;
    fs::write(JSON_DATA_OUTPUT_FILE, json_output)?;
    println!("Successfully generated structured data: {}", JSON_DATA_OUTPUT_FILE);

    // 3. Setup Tera
    let mut tera = Tera::new(TEMPLATE_DIR)?;
    tera.autoescape_on(vec![]); // Disable autoescaping for markdown output and Rust code

    // 4. Render Markdown report
    render_markdown_report(&type_usages, &mut tera)?;

    // 5. Render Macro Refactor code
    render_macro_refactor_code(&type_usages, &mut tera)?;

    Ok(())
}

fn render_markdown_report(type_usages: &[TypeUsage], tera: &mut Tera) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("type_usages", type_usages);
    let rendered = tera.render(MARKDOWN_REPORT_TEMPLATE_NAME, &context)?;
    fs::write(MARKDOWN_REPORT_OUTPUT_FILE, rendered)?;
    println!("Successfully generated Markdown report: {}", MARKDOWN_REPORT_OUTPUT_FILE);
    Ok(())
}

fn render_macro_refactor_code(type_usages: &[TypeUsage], tera: &mut Tera) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("type_usages", type_usages);
    let rendered = tera.render(MACRO_REFACTOR_TEMPLATE_NAME, &context)?;
    fs::write(MACRO_REFACTOR_OUTPUT_FILE, rendered)?;
    println!("Successfully generated macro refactor code: {}", MACRO_REFACTOR_OUTPUT_FILE);
    Ok(())
}


fn parse_type_usage_logs() -> Result<Vec<TypeUsage>, Box<dyn std::error::Error>> {
    let raw_file = fs::File::open(RAW_LOG_FILE)?;
    let labeled_file = fs::File::open(LABELED_LOG_FILE)?;

    let raw_lines = io::BufReader::new(raw_file).lines();
    let labeled_lines = io::BufReader::new(labeled_file).lines();

    let mut type_usages = Vec::new();

    // The TARGET_FILE is defined in analyze_and_label_types.sh, need to be careful if it changes
    let default_file_path = "submodules/rust/compiler/rustc_expand/src/expand.rs".to_string();

    // Regex for various patterns to extract lifetimes and DRT
    // For `pub fn new(cx: &'a mut crate::base::ExtCtxt<'b, DRT>, monotonic: bool) -> Self {`
    let re_ext_ctxt_new_fn_cx = Regex::new(r"cx: &'(\w+) mut crate::base::ExtCtxt<'(\w+),\s*(\w+)>")?;

    // For `InvocationCollector<'a, 'b, DRT>`
    let re_invocation_collector_type = Regex::new(r"InvocationCollector<('?\w+),\s*('?\w+),\s*(\w+)>")?;

    // For `impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> MacroExpander`
    let re_macro_expander_impl = Regex::new(r"impl<('?\w+),\s*('?\w+),\s*(\w+):\s*OpaqueDeriveResolution\s*\+\s*'static>\s*MacroExpander")?;

    // For `.map(|DeriveResolution { path, item, exts: _, is_const }| {`
    // Fix: escape '|' and be more precise about the full match
    let re_derive_resolution_map = Regex::new(r"\.map\|\s*DeriveResolution\s*\{(?P<pattern>[^}]+)\}\|\s*\{")?;

    // For `MacroRulesMacroExpander<DRT>`
    let re_macro_rules_expander = Regex::new(r"MacroRulesMacroExpander<(\w+)>")?;

    // For `pub type CollectorABDRT<'a, 'b, DRT> = InvocationCollector<'a, 'b, DRT>;`
    let re_collector_ab_drt_alias = Regex::new(r"pub type CollectorABDRT<('?\w+),\s*('?\w+),\s*(\w+)> = InvocationCollector<('?\w+),\s*('?\w+),\s*(\w+)>;")?;

    // For `pub type ExpandContext<'b, DRT> = ExtCtxt<'b, DRT>;`
    let re_expand_context_alias = Regex::new(r"pub type ExpandContext<('?\w+),\s*(\w+)> = ExtCtxt<('?\w+),\s*(\w+)>;")?;

    // For `pub type RefAMutExpandContext_B_DRT<'a, 'b, DRT> = &'a mut crate::base::ExpandContext<'b, DRT>;`
    let re_ref_a_mut_expand_context_alias = Regex::new(r"pub type RefAMutExpandContext_B_DRT<('?\w+),\s*('?\w+),\s*(\w+)> = &'(\w+) mut crate::base::ExpandContext<('?\w+),\s*(\w+)>;")?;


    for (raw_line_res, labeled_line_res) in raw_lines.zip(labeled_lines) {
        let raw_line = raw_line_res?;
        let labeled_line = labeled_line_res?;

        // Raw line format: {line_number}:{code_context}
        let mut parts = raw_line.splitn(2, ':');
        let line_number_str = parts.next().ok_or("Invalid raw log line format")?;
        let code_context = parts.next().ok_or("Invalid raw log line format")?;
        let line_number = line_number_str.parse::<u32>()?;

        let original_name = find_original_name(&code_context, &labeled_line);
        let labeled_name = find_labeled_name(&labeled_line);

        let mut lifetime_a = None;
        let mut lifetime_b = None;
        let mut drt_type = None;
        let mut destructuring_pattern = None;


        // Try to extract arguments using regex. Order matters for specificity.
        if let Some(caps) = re_ref_a_mut_expand_context_alias.captures(&code_context) {
            lifetime_a = caps.get(1).map(|m| m.as_str().to_string());
            lifetime_b = caps.get(2).map(|m| m.as_str().to_string());
            drt_type = caps.get(3).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_expand_context_alias.captures(&code_context) {
            lifetime_b = caps.get(1).map(|m| m.as_str().to_string());
            drt_type = caps.get(2).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_collector_ab_drt_alias.captures(&code_context) {
            lifetime_a = caps.get(1).map(|m| m.as_str().to_string());
            lifetime_b = caps.get(2).map(|m| m.as_str().to_string());
            drt_type = caps.get(3).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_ext_ctxt_new_fn_cx.captures(&code_context) {
            lifetime_a = caps.get(1).map(|m| m.as_str().to_string());
            lifetime_b = caps.get(2).map(|m| m.as_str().to_string());
            drt_type = caps.get(3).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_invocation_collector_type.captures(&code_context) {
            lifetime_a = caps.get(1).map(|m| m.as_str().to_string());
            lifetime_b = caps.get(2).map(|m| m.as_str().to_string());
            drt_type = caps.get(3).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_macro_expander_impl.captures(&code_context) {
            lifetime_a = caps.get(1).map(|m| m.as_str().to_string());
            lifetime_b = caps.get(2).map(|m| m.as_str().to_string());
            drt_type = caps.get(3).map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_derive_resolution_map.captures(&code_context) {
            destructuring_pattern = caps.name("pattern").map(|m| m.as_str().to_string());
        } else if let Some(caps) = re_macro_rules_expander.captures(&code_context) {
            drt_type = caps.get(1).map(|m| m.as_str().to_string());
        }


        type_usages.push(TypeUsage {
            original_name,
            labeled_name,
            file_path: default_file_path.clone(),
            line_number,
            context: code_context.trim().to_string(),
            lifetime_a,
            lifetime_b,
            drt_type,
            destructuring_pattern,
        });
    }

    Ok(type_usages)
}

fn find_original_name(raw_context: &str, labeled_context: &str) -> String {
    let raw_words: Vec<&str> = raw_context.split_whitespace().collect();
    let labeled_words: Vec<&str> = labeled_context.split_whitespace().collect();

    for (i, raw_word) in raw_words.iter().enumerate() {
        if i < labeled_words.len() && raw_word != &labeled_words[i] {
            return raw_word.to_string().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
        }
    }
    raw_context.to_string()
}

fn find_labeled_name(labeled_context: &str) -> String {
    if labeled_context.contains("_EXT_CTXT_ALIAS_") {
        return "_EXT_CTXT_ALIAS_".to_string();
    }
    if labeled_context.contains("_INVOCATION_COLLECTOR_ALIAS_") {
        return "_INVOCATION_COLLECTOR_ALIAS_".to_string();
    }
    if labeled_context.contains("_DERIVE_RESOLUTION_ALIAS_") {
        return "_DERIVE_RESOLUTION_ALIAS_".to_string();
    }
    if labeled_context.contains("_DRT_GENERIC_PARAM_") {
        return "_DRT_GENERIC_PARAM_".to_string();
    }
    labeled_context.to_string()
}