use std::sync::Arc;
use miette::{LabeledSpan, MietteDiagnostic, Report, Severity};

#[derive(Debug, Default)]
pub struct ErrorContext {
    diagnostics: Vec<MietteDiagnostic>,
}

impl ErrorContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_diagnostics(&mut self, loc: crate::Span, message: String, severity: Severity) {
        self.diagnostics.push(MietteDiagnostic {
            labels: Some(vec![LabeledSpan::new_primary_with_span(
                None,
                loc.0..loc.1,
            )]),
            severity: Some(severity),
            code: None,
            help: None,
            message,
            url: None,
        });
    }

    pub(crate) fn add_error(&mut self, loc: crate::Span, error: String) {
        self.add_diagnostics(loc, error, Severity::Error);
    }

    pub(crate) fn add_warning(&mut self, loc:  crate::Span, warning: String) {
        self.add_diagnostics(loc, warning, Severity::Warning);
    }

    pub(crate) fn sort_errors(&mut self) {
        self.diagnostics
            .sort_by_key(|d| d.labels.as_ref().unwrap()[0].offset());
    }

    pub fn print(&self, source: &str) {
        let source = Arc::new(source.to_string());
        for diag in &self.diagnostics {
            let report = Report::new(diag.clone()).with_source_code(source.clone());
            println!("{:?}", report);
        }
    }
}