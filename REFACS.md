# Guide for a Better Codebase

This document outlines a series of recommended refactoring steps to improve the structure, maintainability, and scalability of the `dispatch` application. The primary goal is to move from a monolithic UI-centric design to a more organized, state-driven architecture.

## 1. Introduce a Centralized State and Action/Message System

The current approach of cloning a `WindowWidgets` struct for every event handler is inefficient and creates tight coupling. A better approach is to manage the application's state in a single, central struct and modify it via a system of dispatched actions or messages.

### The Problem:
- All event handlers (closures) need a `clone()` of the entire `WindowWidgets` struct.
- State is scattered and directly manipulated from many different places.
- It's hard to track how and when the state changes, making debugging difficult.

### The Solution: A Model-View-Update (MVU) inspired approach.

1.  **Define a central `AppState` struct:** This struct will hold the *data*, not the widgets.

    ```rust
    // in a new file, e.g., src/state.rs
    pub struct AppState {
        pub current_request: RequestState,
        pub history: Vec<HistoryItem>,
        pub active_view: ActiveView,
        // etc.
    }

    pub struct RequestState {
        pub url: String,
        pub method: String,
        pub body: String,
        pub headers: Vec<(String, String)>,
        // ... and so on
    }
    ```

2.  **Create an `Action` enum:** This enum will represent all possible state changes.

    ```rust
    // in e.g., src/actions.rs
    pub enum Action {
        UpdateUrl(String),
        SelectMethod(String),
        SendRequest,
        RequestCompleted(Result<api::RequestResult, api::ApiError>),
        LoadFromHistory(i64),
        NewRequest,
        ClearHistory,
        // etc.
    }
    ```

3.  **Create a central "update" or "reduce" function:** This function takes the current state and an action, and returns the new state.

    ```rust
    // in your main logic file
    pub fn update(state: &mut AppState, action: Action) {
        match action {
            Action::UpdateUrl(url) => {
                state.current_request.url = url;
            }
            Action::SendRequest => {
                // Initiate the API call here
            }
            Action::RequestCompleted(response_result) => {
                // Update state with the response
            }
            // ... handle all other actions
        }
    }
    ```

4.  **Use a `glib::Sender` to dispatch actions:** Your UI event handlers will no longer modify widgets directly. Instead, they will just send an `Action`.

    **Before:**
    ```rust
    send_button.connect_clicked(move |_| {
        let url = w.url_entry.text().to_string();
        w.spinner.start();
        // ... and so on
    });
    ```

    **After:**
    ```rust
    let action_sender = /* get from your app context */;
    send_button.connect_clicked(move |_| {
        action_sender.send(Action::SendRequest).unwrap();
    });
    ```

A main loop would receive these actions, call the `update` function, and then have a separate `view` function that applies the new state to the UI widgets.

## 2. Decouple UI Components from Global State

Instead of a giant `build` function, break down each part of the UI into a self-contained component with its own `build` and `update` logic.

### The Problem:
- `window.rs` knows about the internal widgets of `sidebar.rs`, `request_bar.rs`, etc.
- To change the sidebar, you have to modify `window.rs`.
- Event handlers for all components are defined in the top-level `window.rs`.

### The Solution: Create a `Component` Trait.

Define a trait that standardizes how UI components are updated.

```rust
// in src/ui/mod.rs
pub trait Component {
    // Renders the component based on the global state
    fn render(&self, state: &AppState);

    // Wires up signals to dispatch actions
    fn connect_signals(&self, sender: glib::Sender<Action>);
}
```

Your `window.rs` then becomes much simpler. It holds a list of `Box<dyn Component>` and iterates over them.

## 3. Abstract Repetitive Logic

There are several instances of repeated logic that can be extracted into helper functions or traits.

### Example 1: Finding Selected Method

The `match` statement to get the method string from a dropdown index is used in multiple places.

**Before (repeated logic):**
```rust
let method_idx = match item.method.as_str() {
    "GET" => 0, "POST" => 1, /*...*/
};
w_sidebar.method_dropdown.set_selected(method_idx);

// and elsewhere...
let selected_method = w.method_dropdown.selected();
let method_str = match selected_method {
    0 => "GET", 1 => "POST", /*...*/
};
```

**Solution:** Create a dedicated type or helper.

```rust
// In a new file like `src/http.rs`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method { GET, POST, PUT, PATCH, DELETE }

impl Method {
    pub fn from_index(index: u32) -> Self { /* ... */ }
    pub fn to_index(&self) -> u32 { /* ... */ }
    pub fn as_str(&self) -> &'static str { /* ... */ }
}

// You can even implement From/Into for conversions
impl From<u32> for Method { /* ... */ }
```

### Example 2: Styling the Status Label

The logic to set the "success" or "error" class is repeated.

**Solution:** Create a helper function in your `status_bar` component.

```rust
// in src/ui/status_bar.rs
impl StatusBar { // Assuming you create a struct for your component
    pub fn set_status(&self, status: &Status) {
        match status {
            Status::Success(message) => {
                self.label.set_text(message);
                self.label.add_css_class(config::CLASS_SUCCESS);
                self.label.remove_css_class(config::CLASS_ERROR);
            }
            Status::Error(message) => {
                self.label.set_text(message);
                self.label.add_css_class(config::CLASS_ERROR);
                self.label.remove_css_class(config::CLASS_SUCCESS);
            }
            // ... other statuses
        }
    }
}
```

## 4. Improve Error Handling in API

The `api::perform_request` function mixes successful and error states in the same `RequestResult` struct, using a boolean `is_error` flag. This is a common anti-pattern in Rust. It's better to use `Result`.

### The Problem:
- The caller has to check `is_error` before using the other fields.
- The fields in `RequestResult` might be meaningless in an error case (`time`, `size`, etc.).

### The Solution: Use a dedicated `ApiError` enum and return `Result`.

```rust
// in src/api/mod.rs
pub struct SuccessResponse {
    pub body: String,
    pub status: String,
    pub time: String,
    pub size: String,
    pub headers: String,
}

pub enum ApiError {
    RequestFailed(reqwest::Error),
    JsonParseFailed,
    InvalidHeader,
    // etc.
}

// The function signature changes
pub fn perform_request(...) -> Result<SuccessResponse, ApiError> {
    // ...
    match result {
        Ok(response) => {
            // ...
            Ok(SuccessResponse { /* ... */ })
        }
        Err(e) => Err(ApiError::RequestFailed(e)),
    }
}
```

This makes the call site much cleaner and leverages Rust's type system to ensure errors are handled properly. Your `Action` enum would then carry this result: `RequestCompleted(Result<SuccessResponse, ApiError>)`.
