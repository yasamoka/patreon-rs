use bon::Builder;

pub(crate) trait Query {
    fn query_params(&self) -> Option<String>;
}

impl<T: QueryImpl> Query for T {
    fn query_params(&self) -> Option<String> {
        const PAGE_CURSOR_KEY: &str = "page%5Bcursor%5D";
        const PAGE_COUNT_KEY: &str = "page%5Bcount%5D";

        match (
            self.cursor(),
            self.page_size()
                .map(|page_size| page_size.min(Self::max_page_size())),
        ) {
            (None, None) => None,
            (None, Some(page_size)) => Some(format!("{PAGE_COUNT_KEY}={page_size}")),
            (Some(cursor), None) => Some(format!("{PAGE_CURSOR_KEY}={cursor}")),
            (Some(cursor), Some(page_size)) => Some(format!(
                "{PAGE_CURSOR_KEY}={cursor}&{PAGE_COUNT_KEY}={page_size}",
            )),
        }
    }
}

trait QueryImpl {
    fn max_page_size() -> u32;

    fn cursor(&self) -> Option<&String>;

    fn page_size(&self) -> Option<u32>;
}

/// Query parameters for listing members.
#[derive(Builder, Debug, Clone, Default)]
pub struct MembersQuery {
    /// Cursor (for pagination).
    pub cursor: Option<String>,
    /// Page size (max 1000).
    pub page_size: Option<u32>,
}

impl QueryImpl for MembersQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }

    fn page_size(&self) -> Option<u32> {
        self.page_size
    }

    fn max_page_size() -> u32 {
        1000
    }
}

/// Query parameters for listing posts.
#[derive(Builder, Debug, Clone, Default)]
pub struct PostsQuery {
    /// Cursor (for pagination).
    pub cursor: Option<String>,
    /// Page size (max 100).
    pub page_size: Option<u32>,
}

impl QueryImpl for PostsQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }

    fn page_size(&self) -> Option<u32> {
        self.page_size
    }

    fn max_page_size() -> u32 {
        100
    }
}
