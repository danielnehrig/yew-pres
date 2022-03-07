pub mod context {
    use serde::Deserialize;

    #[derive(Clone, Deserialize, Debug, PartialEq)]
    pub struct MovieContext {
        pub id: usize,
        pub title: String,
        pub speaker: String,
        pub url: String,
    }
}
