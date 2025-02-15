use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn RestrictionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let ctx = use_dev_context(scope);
    let generic = create_signal(scope, true);
    let presentation_signal = ctx.presentation_signal();
    let last_sample_id = create_signal(scope, presentation_signal.get_untracked().opt_sample_id());
    let toggle_restriction_kind_handler = ctx.toggle_restriction_kind_handler();
    let restriction_kind = memo!(
        scope,
        move || match presentation_signal.get().is_specific() {
            true => "SPECIFIC",
            false => "GENERIC",
        }
        .to_string(),
        presentation_signal
    );
    view! {
        scope,
        div (class="RestrictionView disable-select") {
            div (
                class="RestrictionKind",
                on:click=toggle_restriction_kind_handler
            ) {
                (restriction_kind.get())
            }
             label (
                id="sample-id-name",
            ) {
                "sample id = "
            }
            label (
                id="sample-id-value",
                on:click=ctx.set_restriction_from_dialog_handler()
            ) {
                (presentation_signal.get().sample_id().0)
            }
        }
    }
}
