#!/bin/bash
# Monster Group Solana Validator - Production Deployment Validation
# Checks for placeholder data, missing binaries, and security issues

set -e

echo "🔍 MONSTER GROUP PRODUCTION DEPLOYMENT VALIDATION"
echo "================================================="
echo ""

VALIDATION_ERRORS=0
VALIDATION_WARNINGS=0

# Function to report errors
report_error() {
    echo "❌ ERROR: $1"
    ((VALIDATION_ERRORS++))
}

# Function to report warnings  
report_warning() {
    echo "⚠️  WARNING: $1"
    ((VALIDATION_WARNINGS++))
}

# Function to report success
report_success() {
    echo "✅ $1"
}

echo "🔐 CHECKING SECRETS CONFIGURATION"
echo "================================="

# Check for placeholder secrets
SECRETS_FILE="ai-agent-terraform/environments/monster-solana-validator-nix/secrets.yaml"
if [ -f "$SECRETS_FILE" ]; then
    if grep -q "CHANGE_ME\|REPLACE_WITH" "$SECRETS_FILE"; then
        report_error "Placeholder secrets found in $SECRETS_FILE"
        echo "   Run: cp $SECRETS_FILE.example $SECRETS_FILE"
        echo "   Then replace all REPLACE_WITH_* values with real secrets"
    else
        report_success "No placeholder secrets found"
    fi
    
    # Check if secrets are encrypted
    if grep -q "sops:" "$SECRETS_FILE"; then
        report_success "Secrets file is encrypted with ROPS"
    else
        report_warning "Secrets file is not encrypted - run: rops encrypt $SECRETS_FILE"
    fi
else
    report_error "Secrets file not found: $SECRETS_FILE"
    echo "   Copy from $SECRETS_FILE.example and configure real secrets"
fi

echo ""
echo "🔨 CHECKING RUST BINARIES"
echo "========================="

# Check for built binaries
RUST_DIR="tools/rust-71-parts"
REQUIRED_BINARIES=(
    "monster_cloudformation_generator"
    "monster_oci_generator" 
    "solana_validator_demo"
    "monster_nar_dataset_generator"
)

for binary in "${REQUIRED_BINARIES[@]}"; do
    if [ -f "$RUST_DIR/target/release/$binary" ]; then
        report_success "Binary built: $binary"
    elif [ -f "$RUST_DIR/target/debug/$binary" ]; then
        report_warning "Binary only in debug mode: $binary"
        echo "   Run: cd $RUST_DIR && cargo build --release --bin $binary"
    else
        report_error "Binary missing: $binary"
        echo "   Run: cd $RUST_DIR && cargo build --release --bin $binary"
    fi
done

echo ""
echo "🌐 CHECKING DEPLOYMENT CONFIGURATIONS"
echo "===================================="

# Check CloudFormation generator
if [ -f "$RUST_DIR/src/bin/monster_cloudformation_generator.rs" ]; then
    if grep -q "ami-0c02fb55956c7d316" "$RUST_DIR/src/bin/monster_cloudformation_generator.rs"; then
        report_warning "Hardcoded AMI ID found in CloudFormation generator"
        echo "   Consider using dynamic AMI lookup for multi-region support"
    else
        report_success "No hardcoded AMI IDs in CloudFormation generator"
    fi
fi

# Check OCI generator
if [ -f "$RUST_DIR/src/bin/monster_oci_generator.rs" ]; then
    if grep -q "us-ashburn-1" "$RUST_DIR/src/bin/monster_oci_generator.rs"; then
        report_warning "Hardcoded region found in OCI generator"
        echo "   Ensure region is configurable via variables"
    else
        report_success "No hardcoded regions in OCI generator"
    fi
fi

echo ""
echo "🛡️  CHECKING SECURITY CONFIGURATION"
echo "==================================="

# Check for overly permissive security groups
TERRAFORM_FILES=$(find ai-agent-terraform -name "*.tf" 2>/dev/null || true)
if [ -n "$TERRAFORM_FILES" ]; then
    if echo "$TERRAFORM_FILES" | xargs grep -l "0.0.0.0/0" >/dev/null 2>&1; then
        report_warning "Found 0.0.0.0/0 CIDR blocks in security configurations"
        echo "   Review and restrict access to minimum required IPs"
    else
        report_success "No overly permissive CIDR blocks found"
    fi
fi

# Check for Monster Group factor validation
if grep -r "monster_factor.*71" tools/rust-71-parts/src/ >/dev/null 2>&1; then
    report_success "Monster Group factor 71 (sentinel) properly configured"
else
    report_warning "Monster Group factor validation may be missing"
fi

echo ""
echo "📦 CHECKING NAR DATASET GENERATION"
echo "=================================="

# Check NAR generation capability
if [ -f "$RUST_DIR/generate-nar-dataset.sh" ]; then
    report_success "NAR dataset generation script found"
    
    # Test if it can run (dry run)
    if cd "$RUST_DIR" && ./generate-nar-dataset.sh >/dev/null 2>&1; then
        report_success "NAR dataset generation script executes successfully"
    else
        report_warning "NAR dataset generation script has execution issues"
    fi
    cd - >/dev/null
else
    report_error "NAR dataset generation script missing"
fi

echo ""
echo "🔧 CHECKING DEPENDENCIES"
echo "========================"

# Check required tools
REQUIRED_TOOLS=("git" "openssl" "jq")
for tool in "${REQUIRED_TOOLS[@]}"; do
    if command -v "$tool" >/dev/null 2>&1; then
        report_success "$tool is available"
    else
        report_error "$tool is required but not found"
    fi
done

# Check optional tools
OPTIONAL_TOOLS=("nix" "cargo" "terraform" "aws" "oci" "ipfs" "age")
for tool in "${OPTIONAL_TOOLS[@]}"; do
    if command -v "$tool" >/dev/null 2>&1; then
        report_success "$tool is available"
    else
        report_warning "$tool is not available (optional for some deployments)"
    fi
done

echo ""
echo "📊 VALIDATION SUMMARY"
echo "===================="
echo "Errors: $VALIDATION_ERRORS"
echo "Warnings: $VALIDATION_WARNINGS"

if [ $VALIDATION_ERRORS -eq 0 ]; then
    if [ $VALIDATION_WARNINGS -eq 0 ]; then
        echo ""
        echo "🎉 VALIDATION PASSED!"
        echo "===================="
        echo "✅ All checks passed - ready for production deployment"
        echo "🏛️  Monster Group Solana Validator is production-ready"
        exit 0
    else
        echo ""
        echo "⚠️  VALIDATION PASSED WITH WARNINGS"
        echo "==================================="
        echo "✅ No critical errors found"
        echo "⚠️  $VALIDATION_WARNINGS warnings should be addressed"
        echo "🏛️  Monster Group Solana Validator can be deployed with caution"
        exit 0
    fi
else
    echo ""
    echo "❌ VALIDATION FAILED"
    echo "==================="
    echo "❌ $VALIDATION_ERRORS critical errors must be fixed"
    echo "⚠️  $VALIDATION_WARNINGS warnings should be addressed"
    echo "🛑 DO NOT deploy to production until all errors are resolved"
    exit 1
fi
