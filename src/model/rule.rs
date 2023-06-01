pub mod rule{
    pub enum Category {
        BestPractices,
        Security,
        Safety,
        Design,
        CodeStyle
    }

    pub enum Severity {
        Critical,
        Warning,
        Info
    }

    pub struct Rule {
        pub name: String,
        pub content: String,
        pub severity: Severity,
        pub category: Category
    }

}
