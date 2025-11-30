use anyhow::{Result, Context};
use std::path::{PathBuf, Path};
use crate::{Factory, FactoryBlock}; // Correct import for Factory and FactoryBlock trait
use std::process::Command; // Added
use std::fs; // Added
use chrono::Local; // Added for timestamps
use serde_json::Value; // Added for parsing flake.lock
use quote::quote; // Added for Rust code generation
use crate::factory_blocks::ai_llm_blocks::{NotebookLMBlock, LlmBlock}; // Needed for AutomatedReportVideoAudioWorkflowBlock and InvokeAIConversionBlock
use crate::factory_blocks::CodeEvaluatorBlock; // Needed for InvokeAIConversionBlock
//use crate::factory_blocks::RustcBlock; // Needed for InvokeAIConversionBlock
use crate::factory_blocks::rustc_meta_blocks::RustcBlock; // Needed for InvokeAIConversionBlock
use serde::{Deserialize, Serialize}; // Add this import
// use super::meme_comm_blocks; // Not used in this file


#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct ImageGeneratorBlock;
impl FactoryBlock for ImageGeneratorBlock {
    fn name(&self) -> &'static str { "Image Generator API" }
    fn cost(&self) -> u32 { 150 } // Cost for image generation
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Image Generator API activated! Generating images based on prompts and parameters.");
        factory.points += 40;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct AudioGeneratorBlock;
impl FactoryBlock for AudioGeneratorBlock {
    fn name(&self) -> &'static str { "Audio Generator API" }
    fn cost(&self) -> u32 { 120 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Audio Generator API activated! Synthesizing audio based on inputs.");
        factory.points += 30;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct VideoGeneratorBlock;
impl FactoryBlock for VideoGeneratorBlock {
    fn name(&self) -> &'static str { "Video Generator API" }
    fn cost(&self) -> u32 { 200 } // High cost for video generation
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Video Generator API activated! Generating video sequences from provided data or instructions.");
        factory.points += 50;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct InvokeAIConversionBlock;
impl FactoryBlock for InvokeAIConversionBlock {
    fn name(&self) -> &'static str { "Invoke.AI to Rust Converter" }
    fn cost(&self) -> u32 { 800 } // Very high cost for complex cross-language conversion
    fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
        println!("Invoke.AI to Rust Converter activated! Ingesting Node.js/Python features from Invoke.AI and converting them to pure Rust.");
        // This block would leverage various analysis and code generation blocks.
        // For example:
        // ReadFileBlock.execute(factory, current_crate_path)?; // Read the Invoke.AI source
        // CodeEvaluatorBlock.execute(factory, current_crate_path)?; // Analyze its structure
        // LLMBlock.execute(factory, current_crate_path)?; // Use LLM for translation suggestions
        // RustcBlock.execute(factory, current_crate_path)?; // Compile generated Rust code
        factory.points += 250;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct ReportGeneratorBlock;
impl FactoryBlock for ReportGeneratorBlock {
    fn name(&self) -> &'static str { "Report Generator" }
    fn cost(&self) -> u32 { 70 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Report Generator activated! Compiling data and insights into comprehensive reports.");
        factory.points += 20;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct AutomatedReportVideoAudioWorkflowBlock;
impl FactoryBlock for AutomatedReportVideoAudioWorkflowBlock {
    fn name(&self) -> &'static str { "Automated Report/Media Workflow" }
    fn cost(&self) -> u32 { 500 } // High cost for complex automation
    fn execute(&self, factory: &mut Factory, current_crate_path: &PathBuf) -> Result<()> {
        println!("\n--- Automated Report/Media Workflow Activated! ---");
        println!("Orchestrating code export, NotebookLM integration, and multimedia generation.");

        // Simulate code export to text
        println!("1. Exporting code to text...");
        // Placeholder for actual code export. Could use CrateExporterBlock or a specialized tool.
        // For now, simulate.
        factory.points += 20;

        // Simulate loading into NotebookLM and report generation
        println!("2. Loading into NotebookLM and generating reports...");
        NotebookLMBlock.execute(factory, current_crate_path)?;
        ReportGeneratorBlock.execute(factory, current_crate_path)?;

        // Simulate video and audio generation from reports
        println!("3. Generating video and audio from reports...");
        VideoGeneratorBlock.execute(factory, current_crate_path)?;
        AudioGeneratorBlock.execute(factory, current_crate_path)?;

        println!("--- Automated Report/Media Workflow Completed! ---");
        factory.points += 150;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct LaTeXProcessorBlock;
impl FactoryBlock for LaTeXProcessorBlock {
    fn name(&self) -> &'static str { "LaTeX Processor" }
    fn cost(&self) -> u32 { 60 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("LaTeX Processor activated! Tangling and weaving LaTeX/TeX documents.");
        
        // This is simplified execution from MermaidIntegrationBlock in rusttycoon
        // In a real scenario, this would process LaTeX files, maybe generate PDFs, etc.
        let mermaid_diagram = factory.render_factory_floor(); // Assuming a render_factory_floor method
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let output_dir = PathBuf::from("./generated_latex");
        fs::create_dir_all(&output_dir)?;

        let tex_file_path = output_dir.join(format!("document_{}.tex", timestamp));
        fs::write(&tex_file_path, &mermaid_diagram)?;
        println!("Dummy LaTeX document saved to {:?}", tex_file_path);

        // Simulate pdflatex command
        let pdf_file_path = output_dir.join(format!("document_{}.pdf", timestamp));
        let pdf_output = Command::new("pdflatex")
            .arg("-output-directory").arg(&output_dir)
            .arg(&tex_file_path)
            .output()
            .context("Failed to execute pdflatex command. Is pdflatex installed and in PATH?")?;
        
        if pdf_output.status.success() {
            println!("Generated PDF: {:?}", pdf_file_path);
            factory.generated_assets.push(pdf_file_path);
        } else {
            eprintln!("Failed to generate PDF: {}", String::from_utf8_lossy(&pdf_output.stderr));
        }

        factory.points += 25;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct MermaidBlock;
impl FactoryBlock for MermaidBlock {
    fn name(&self) -> &'static str { "Mermaid" }
    fn cost(&self) -> u32 { 10 }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct MermaidIntegrationBlock;
impl FactoryBlock for MermaidIntegrationBlock {
    fn name(&self) -> &'static str { "Mermaid Integration" }
    fn cost(&self) -> u32 { 10 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Mermaid Integration block executed. Generating Mermaid diagram and images.");
        let mermaid_diagram = factory.render_factory_floor();
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let output_dir = PathBuf::from("./generated_assets");
        fs::create_dir_all(&output_dir)?;

        let mmd_file_path = output_dir.join(format!("factory_floor_{}.mmd", timestamp));
        fs::write(&mmd_file_path, &mermaid_diagram)?;
        println!("Mermaid diagram saved to {:?}", mmd_file_path);

        // Generate SVG
        let svg_file_path = output_dir.join(format!("factory_floor_{}.svg", timestamp));
        let svg_output = Command::new("mmdc")
            .arg("-i").arg(&mmd_file_path)
            .arg("-o").arg(&svg_file_path)
            .output()
            .context("Failed to execute mmdc command for SVG. Is mmdc installed and in PATH?")?;
        
        if svg_output.status.success() {
            println!("Generated SVG: {:?}", svg_file_path);
            factory.generated_assets.push(svg_file_path);
        } else {
            eprintln!("Failed to generate SVG: {}", String::from_utf8_lossy(&svg_output.stderr));
        }

        // Generate PNG
        let png_file_path = output_dir.join(format!("factory_floor_{}.png", timestamp));
        let png_output = Command::new("mmdc")
            .arg("-i").arg(&mmd_file_path)
            .arg("-o").arg(&png_file_path)
            .output()
            .context("Failed to execute mmdc command for PNG. Is mmdc installed and in PATH?")?;

        if png_output.status.success() {
            println!("Generated PNG: {:?}", png_file_path);
            factory.generated_assets.push(png_file_path);
        } else {
            eprintln!("Failed to generate PNG: {}", String::from_utf8_lossy(&png_output.stderr));
        }

        // Clean up temporary .mmd file
        fs::remove_file(&mmd_file_path)?;

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)] // Add Serialize, Deserialize
#[typetag::serde] // Add typetag
pub struct ArchiveOrgBlock;
impl FactoryBlock for ArchiveOrgBlock {
    fn name(&self) -> &'static str { "Archive.org Downloader" }
    fn cost(&self) -> u32 { 30 }
    fn execute(&self, factory: &mut Factory, _current_crate_path: &PathBuf) -> Result<()> {
        println!("Archive.org Downloader activated! Accessing historical data and archives from Archive.org.");
        factory.points += 30;
        Ok(())
    }
}

