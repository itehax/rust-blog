use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn RssFeed(cx: Scope) -> impl IntoView {
    cfg_if! {
      if #[cfg(feature="ssr")]{
        use http::{StatusCode, HeaderName,HeaderValue};
        use leptos_axum::ResponseOptions;

        let response = use_context::<ResponseOptions>(cx);
        if let Some(response) = response {
          response.set_status(StatusCode::OK);
          let name = HeaderName::from_bytes(b"Content-Type").unwrap();
          let value = HeaderValue::from_bytes(b"application/xml").unwrap();
          response.insert_header(name, value);
        }
      }
    }
    view! { cx,

    }
}
