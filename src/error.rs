use in_place::InPlaceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ConfitError {

    #[error("issue accessing the file")]
    IssueReadingFile(#[from] std::io::Error),

    #[error("issue accessing the file")]
    InPlaceIssueReadingFile(#[from] InPlaceError),
}