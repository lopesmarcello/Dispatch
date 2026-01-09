use crate::api::{ApiError, ApiResponse};
use crate::models::Method;

#[derive(Debug, Clone)]
pub enum Action {
    // User Inputs
    UpdateUrl(String),
    UpdateMethod(Method),
    UpdateBody(String),
    UpdateHeaders(Vec<(String, String)>),

    // Commands
    SendRequest,
    NewRequest,
    ClearHistory,
    LoadHistoryItem(i64),

    // Async
    RequestStarted,
    RequestCompleted(Result<ApiResponse, ApiError>),
    HistorySaved(i64),

    // Collections
    CreateCollection(String),
    SelectCollection(i64),
    SaveRequestToCollection {col_id: i64, name: String},
    LoadCollectionItem(i64),

}
