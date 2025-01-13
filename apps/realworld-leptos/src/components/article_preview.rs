use super::buttons::{ButtonFav, ButtonFollow};
use crate::auth;
use leptos::prelude::*;

pub type ArticleSignal = RwSignal<crate::models::Article>;

#[component]
pub fn ArticlePreviewList(articles: ReadSignal<Vec<crate::models::Article>>) -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading Articles"</p> }>
            <ErrorBoundary fallback=|_| {
                view! { <p class="error-messages text-xs-center">"Something went wrong."</p> }
            }>
                <For
                    each=move || articles.get()
                    key=|article| article.slug.clone()
                    children=move |article: crate::models::Article| {
                        let article = RwSignal::new(article);
                        view! { <ArticlePreview article=article /> }
                    }
                />
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn ArticlePreview(article: ArticleSignal) -> impl IntoView {
    view! {
        <div class="article-preview">
            <ArticleMeta article=article is_preview=true />
            <a
                href=move || format!("/article/{}", article.with(|x| x.slug.clone()))
                class="preview-link"
            >
                <h1>{move || article.with(|x| x.title.to_string())}</h1>
                <p>{move || article.with(|x| x.description.to_string())}</p>
                <span>"Read more..."</span>
                <Show when=move || article.with(|x| !x.tag_list.is_empty())>
                    <ul class="tag-list">
                        <For
                            each=move || {
                                article.with(|x| x.tag_list.clone().into_iter().enumerate())
                            }
                            key=|(i, _)| *i
                            children=move |(_, tag): (usize, String)| {
                                view! { <li class="tag-default tag-pill tag-outline">{tag}</li> }
                            }
                        />
                    </ul>
                </Show>
            </a>
        </div>
    }
}

#[component]
pub fn ArticleMeta(article: ArticleSignal, is_preview: bool) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let editor_ref = move || format!("/editor/{}", article.with(|x| x.slug.to_string()));
    let profile_ref = move || {
        format!(
            "/profile/{}",
            article.with(|x| x.author.username.to_string())
        )
    };

    view! {
        <div class="article-meta">
            <a href=profile_ref>
                <img src=move || article.with(|x| x.author.image.clone().unwrap_or_default()) />
            </a>
            <div class="info">
                <a href=profile_ref class="author">
                    {move || article.with(|x| x.author.username.to_string())}
                </a>
                <time class="date" datetime=move || article.with(|x| x.created_at.to_string())>
                    {move || article.with(|x| x.created_at.to_string())}
                </time>
            </div>
            <Show
                when=move || is_preview
                fallback=move || {
                    view! {
                        <Show
                            when=move || {
                                auth_context.user.get().map(|x| x.username()).unwrap_or_default()
                                    == article.with(|x| x.author.username.to_string())
                            }
                            fallback=move || {
                                let following = article.with(|x| x.author.following);
                                let (author, _) = signal(
                                    article.with(|x| x.author.username.to_string()),
                                );
                                view! {
                                    <Show
                                        when=move || auth_context.user.with(Option::is_some)
                                        fallback=|| ()
                                    >
                                        <ButtonFav article=article />
                                        <ButtonFollow author following />
                                    </Show>
                                }
                            }
                        >
                            <a class="btn btn-sm btn-outline-secondary" href=editor_ref>
                                <i class="ion-compose"></i>
                                " Edit article"
                            </a>
                            <form class="inline">
                                <input
                                    type="hidden"
                                    name="slug"
                                    value=move || article.with(|x| x.slug.to_string())
                                />
                                <button type="submit" class="btn btn-sm btn-outline-secondary">
                                    <i class="ion-trash-a"></i>
                                    " Delete article"
                                </button>
                            </form>
                        </Show>
                    }
                }
            >
                <ButtonFav article=article />
            </Show>
        </div>
    }
}

#[server(DeleteArticleAction, "/api")]
pub async fn delete_article(slug: String) -> Result<(), ServerFnError> {
    let Some(logged_user) = crate::auth::get_username() else {
        return Err(ServerFnError::ServerError("you must be logged in".into()));
    };
    let redirect_profile = format!("/profile/{logged_user}");

    crate::models::Article::delete(slug, logged_user)
        .await
        .map(move |_| {
            leptos_axum::redirect(&redirect_profile);
        })
        .map_err(|x| {
            let err = format!("Error while deleting an article: {x:?}");
            tracing::error!("{err}");
            ServerFnError::ServerError("Could not delete the article, try again later".into())
        })
}
