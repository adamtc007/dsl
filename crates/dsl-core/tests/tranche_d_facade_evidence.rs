#[test]
fn test_tranche_d_facade_evidence() {
    // 1. SourceSpan::new
    let _ = dsl_core::SourceSpan::new(1, 1, 1, 10);

    // 2. PlanId::new
    let _ = dsl_core::PlanId::new();

    // 3. BindingSlotId::new
    let _ = dsl_core::BindingSlotId::new("slot_a");

    // 4. PopulatedExecutionDag::new
    let _ = dsl_core::PopulatedExecutionDag::new();

    // 5. BindingContext::new
    let _ = dsl_core::BindingContext::new();

    // 6. Span::new
    let _ = dsl_core::Span::new(0, 10);

    // 7. NavDirection::parse
    let _ = dsl_core::NavDirection::parse("up");

    // 8. ViewType::parse
    let _ = dsl_core::ViewType::parse("table");

    // 9. ConfidenceZone::parse
    let _ = dsl_core::ConfidenceZone::parse("high");

    // 10. ExportFormat::parse
    let _ = dsl_core::ExportFormat::parse("json");

    // 11. SearchKeyConfig::parse
    let _ = dsl_core::SearchKeyConfig::parse("key");

    // 12. Location::verb
    let _ = dsl_core::Location::verb("v");

    // 13. ConfigLoader::new
    let _ = dsl_core::ConfigLoader::new("dir");

    // 14. ResolvedTemplate::slot
    fn check_resolved_template_slot(template: &dsl_core::ResolvedTemplate) {
        let _ = template.slot("id");
    }
}
