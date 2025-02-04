use leptos::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn EditorPage() -> impl IntoView {
    let (error_messages, _set_error_messages) = signal(Vec::<String>::new());

    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (tags, set_tags) = signal(String::new());

    view! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <Show when=move || !error_messages.get().is_empty()>
                            <ul class="error-messages">
                                <For
                                    each=move || error_messages.get().into_iter().enumerate()
                                    key=|(index, _)| *index
                                    children=move |(_, message)| {
                                        let message = message.to_string();
                                        view! { <li>{message}</li> }
                                    }
                                />
                            </ul>
                        </Show>
                        <form>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control form-control-lg"
                                        placeholder="Article Title"
                                        bind:value=(title, set_title)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="What's this article about?"
                                        bind:value=(description, set_description)
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in markdown)"
                                        bind:value=(body, set_body)
                                    ></textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Enter tags"
                                        bind:value=(tags, set_tags)
                                    />
                                    <div class="tag-list">
                                        <For
                                            each=move || {
                                                tags.get()
                                                    .split(',')
                                                    .map(|tag| tag.trim().to_string())
                                                    .collect::<Vec<String>>()
                                            }
                                            key=|tag| tag.clone()
                                            children=move |tag| {
                                                view! {
                                                    <span class="tag-default tag-pill">
                                                        <i class="ion-close-round" />
                                                        {tag}
                                                    </span>
                                                }
                                            }
                                        />
                                    </div>
                                </fieldset>
                                <button type="button" class="btn btn-lg pull-xs-right btn-primary">
                                    Publish Article
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
