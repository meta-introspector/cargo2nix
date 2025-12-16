use syn::{self, parse_quote};
use quote::ToTokens;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SystemModuleCategory {
    CoreLogic,
    DatabaseAccess,
    NetworkLayer,
    UserInterface,
    Observability,
    MlTrainingPipeline,
    AwsS3Buckets,
    Custom(u64), // The "bitmask" part
    None, // Default or unclassified
    All,  // Matches everything
}

impl ToTokens for SystemModuleCategory {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = match self {
            SystemModuleCategory::CoreLogic => parse_quote!(SystemModuleCategory::CoreLogic),
            SystemModuleCategory::DatabaseAccess => parse_quote!(SystemModuleCategory::DatabaseAccess),
            SystemModuleCategory::NetworkLayer => parse_quote!(SystemModuleCategory::NetworkLayer),
            SystemModuleCategory::UserInterface => parse_quote!(SystemModuleCategory::UserInterface),
            SystemModuleCategory::Observability => parse_quote!(SystemModuleCategory::Observability),
            SystemModuleCategory::MlTrainingPipeline => parse_quote!(SystemModuleCategory::MlTrainingPipeline),
            SystemModuleCategory::AwsS3Buckets => parse_quote!(SystemModuleCategory::AwsS3Buckets),
            SystemModuleCategory::Custom(val) => {
                quote::quote_spanned! { proc_macro2::Span::call_site() => SystemModuleCategory::Custom(#val) }
            },
            SystemModuleCategory::None => parse_quote!(SystemModuleCategory::None),
            SystemModuleCategory::All => parse_quote!(SystemModuleCategory::All),
        };
        path.to_tokens(tokens);
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VetoTrigger {
    Always,
    Never,
    IfDebugBuild,
    IfReleaseBuild,
    IfFeatureEnabled(String),
}

impl ToTokens for VetoTrigger {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = match self {
            VetoTrigger::Always => parse_quote!(VetoTrigger::Always),
            VetoTrigger::Never => parse_quote!(VetoTrigger::Never),
            VetoTrigger::IfDebugBuild => parse_quote!(VetoTrigger::IfDebugBuild),
            VetoTrigger::IfReleaseBuild => parse_quote!(VetoTrigger::IfReleaseBuild),
            VetoTrigger::IfFeatureEnabled(feature) => {
                quote::quote_spanned! { proc_macro2::Span::call_site() => VetoTrigger::IfFeatureEnabled(#feature.to_string()) }
            },
        };
        path.to_tokens(tokens);
    }
}
