use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth;
use crate::components::ArticleMeta;

#[component]
pub fn ArticlePage() -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let params = hooks::use_params_map();
    let article = LocalResource::new(
        move || {
            let slug = params.get().get("slug").unwrap_or_default();
            crate::models::Article::load_article(slug)
        },
    );

    let title = create_rw_signal(String::from("Loading"));

    view! {
        <Title text=move || title.get() />

        <Suspense fallback=move || view! { <p>"Loading Article"</p> }>
            <ErrorBoundary fallback=|_| {
                view! {
                    <p class="error-messages text-xs-center">
                        "Something went wrong, please try again later."
                    </p>
                }
            }>
                {move || {
                    article
                        .get()
                        .map(move |x| {
                            title.set(x.article.slug.to_string());
                            view! { <ArticleDetail article=x.article.clone() /> }
                        })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn ArticleDetail(article: crate::models::Article) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let article_signal = RwSignal::new(article.clone());
    let tag_list = article.tag_list;

    view! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>{article.title}</h1>
                    <ArticleMeta article=article_signal is_preview=false />
                </div>
            </div>

            <div class="container page">
                <div class="row article-content">
                    <div class="col-md-12">
                        <div inner_html=markdown::to_html(article.body.unwrap_or_default().as_str()) />
                    </div>
                </div>

                <ul class="tag-list">
                    <For
                        each=move || tag_list.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=|(_, a)| {
                            view! { <li class="tag-default tag-pill tag-outline">{a}</li> }
                        }
                    />
                </ul>

                <hr />

                <div class="article-actions">
                    <div class="row" style="justify-content: center;">
                        <ArticleMeta article=article_signal is_preview=false />
                    </div>
                </div>

                <div class="row">
                    <CommentSection article=article_signal />
                </div>
            </div>
        </div>
    }
}


#[component]
fn CommentSection(
    article: crate::components::ArticleSignal,
) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let comments_action = Action::new(|_| {
        async move { todo!() }
    });
    let result = comments_action.version();
    let reset_comment = create_rw_signal("");
    let comments = LocalResource::new(
        move || {
            let slug = article.with(|a| a.slug.to_string());
            reset_comment.set("");
            crate::models::Comment::load_comments(slug)
        },
    );
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        comments_action.dispatch(());
    };

    view! {
        <div class="col-xs-12 col-md-8 offset-md-2">
            <Show when=move || auth_context.is_authenticated.get() fallback=|| ()>
                <form on:submit=on_submit class="card comment-form">
                    <input
                        name="slug"
                        type="hidden"
                        value=move || article.with(|x| x.slug.to_string())
                    />
                    <div class="card-block">
                        <textarea
                            name="body"
                            prop:value=move || reset_comment.get()
                            class="form-control"
                            placeholder="Write a comment..."
                            rows="3"
                        ></textarea>
                    </div>
                    <div class="card-footer">
                        // <img
                        //     src=move || {
                        //         auth_context.username.with(|x| {
                        //             x.as_ref().map(crate::models::User::image).unwrap_or_default()
                        //         })
                        //     }
                        //     class="comment-author-img"
                        // />
                        <button class="btn btn-sm btn-primary" type="submit">
                            "Post Comment"
                        </button>
                    </div>
                </form>
            </Show>
            <Suspense fallback=move || view! { <p>"Loading Comments from the article"</p> }>
                <ErrorBoundary fallback=|_| {
                    view! { <p class="error-messages text-xs-center">"Something went wrong."</p> }
                }>
                    {move || {
                        comments
                            .get()
                            .map(move |res| {
                                view! {
                                    <For
                                        each=move || res.comments.clone().into_iter().enumerate()
                                        key=|(i, _)| *i
                                        children=move |(_, comment)| {
                                            let comment = RwSignal::new(comment);
                                            view! { <Comment comment /> }
                                        }
                                    />
                                }
                            })
                    }}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn Comment(
    comment: RwSignal<crate::models::Comment>,
) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let user_link = move || format!("/profile/{}", comment.with(|x| x.username.to_string()));
    let user_image = move || comment.with(|x| x.user_image.clone().unwrap_or_default());
    let delete_comment_action = Action::new(|_| {
        async move { todo!() }
    });
    let delete_result = delete_comment_action.value();
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        delete_comment_action.dispatch(());
    };

    view! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{move || comment.with(|x| x.body.to_string())}</p>
            </div>
            <div class="card-footer">
                <a href=user_link class="comment-author">
                    <img src=user_image class="comment-author-img" />
                </a>
                " "
                <a href=user_link class="comment-author">
                    {move || comment.with(|x| x.username.to_string())}
                </a>
                <span class="date-posted">
                    {move || comment.with(|x| x.created_at.to_string())}
                </span>
                <Show
                    when=move || {
                        auth_context.username.get().unwrap_or_default()
                            == comment.with(|x| x.username.to_string())
                    }
                    fallback=|| ()
                >
                    <form on:submit=on_submit class="comment-author">
                        <input type="hidden" name="id" value=move || comment.with(|x| x.id) />
                        <button class="btn btn-sm" type="submit">
                            <i class="ion-trash-b"></i>
                        </button>
                    </form>
                </Show>
            </div>
        </div>
    }
}
