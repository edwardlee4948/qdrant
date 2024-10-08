use api::rest::{SearchGroupsRequestInternal, SearchRequestInternal};
use segment::types::{Filter, SearchParams};

use super::StrictModeVerification;
use crate::collection::Collection;
use crate::operations::config_diff::StrictModeConfig;
use crate::operations::types::{CollectionError, SearchRequestBatch};

impl StrictModeVerification for SearchRequestInternal {
    fn indexed_filter_read(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    fn query_limit(&self) -> Option<usize> {
        Some(self.limit)
    }

    fn timeout(&self) -> Option<usize> {
        None
    }

    fn indexed_filter_write(&self) -> Option<&Filter> {
        None
    }

    fn request_search_params(&self) -> Option<&SearchParams> {
        self.params.as_ref()
    }

    fn request_exact(&self) -> Option<bool> {
        None
    }
}

impl StrictModeVerification for SearchRequestBatch {
    fn check_strict_mode(
        &self,
        collection: &Collection,
        strict_mode_config: &StrictModeConfig,
    ) -> Result<(), CollectionError> {
        for search_request in &self.searches {
            search_request
                .search_request
                .check_strict_mode(collection, strict_mode_config)?;
        }
        Ok(())
    }

    fn query_limit(&self) -> Option<usize> {
        None
    }

    fn timeout(&self) -> Option<usize> {
        None
    }

    fn indexed_filter_read(&self) -> Option<&Filter> {
        None
    }

    fn indexed_filter_write(&self) -> Option<&Filter> {
        None
    }

    fn request_exact(&self) -> Option<bool> {
        None
    }

    fn request_search_params(&self) -> Option<&SearchParams> {
        None
    }
}

impl StrictModeVerification for SearchGroupsRequestInternal {
    fn query_limit(&self) -> Option<usize> {
        Some(self.group_request.limit as usize)
    }

    fn indexed_filter_read(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }

    fn request_exact(&self) -> Option<bool> {
        // We already check this in `request_search_params`
        None
    }

    fn request_search_params(&self) -> Option<&SearchParams> {
        self.params.as_ref()
    }

    fn timeout(&self) -> Option<usize> {
        None
    }

    fn indexed_filter_write(&self) -> Option<&Filter> {
        None
    }
}
