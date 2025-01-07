use leptos::prelude::*;
use crate::auth;
use crate::components::{ArticlePreviewList};
use crate::models::{Article, Pagination, Tag};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GetArticlesQueryKey(u32);

async fn get_tags() -> Result<Vec<String>, ServerFnError> {
    // sqlx::query!("SELECT DISTINCT tag FROM ArticleTags")
    //     .map(|x| x.tag)
    //     .fetch_all(crate::database::get_db())
    //     .await
    //     .map_err(|x| {
    //         tracing::error!("problem while fetching tags: {x:?}");
    //         ServerFnError::ServerError("Problem while fetching tags".into())
    //     })
    Ok(vec![])
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    let (articles, set_articles) = signal::<Vec<Article>>(vec![]);
    let (pagination, set_pagination) = signal(Pagination::default());

    let articles_res = LocalResource::new(
        move || Article::load_articles(pagination.get())
    );

    let your_feed_class = move || {
        format!(
            "nav-link {}",
            if auth_context.username.with(Option::is_none) {
                "disabled"
            } else if !pagination.get().get_my_feed() {
                "active"
            } else {
                ""
            }
        )
    };
    let pages = move || {
        let articles_res_opt = articles_res.get();
        if let Some(articles_res) = articles_res_opt.as_deref() {
            let max_page = (articles_res.articles_count as f64 / pagination.get().get_amount() as f64).ceil() as u32;
            (1..=max_page).collect::<Vec<u32>>()
        } else { vec![] }
    };

    // TODO: This is not the right way.
    Effect::new(move || {
        let articles_res_opt = articles_res.get();
        if let Some(articles_res_ref) = articles_res_opt.as_deref() {
            set_articles.set(articles_res_ref.articles.clone());
        } else {
            set_articles.set(vec![]);
        }
    });

    view! {
        <div class="home-page">
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">conduit</h1>
                    <p>"A place to share your knowledge."</p>
                </div>
            </div>

            <div class="container page">
                <div class="row">
                    <div class="col-md-9">
                        <div class="feed-toggle">
                            <ul class="nav nav-pills outline-active">
                                <Show when=move || { auth_context.is_authenticated.get() }>
                                    <li class="nav-item">
                                        <button
                                            class=your_feed_class
                                            class:active=move || { pagination.get().get_my_feed() }
                                            on:click=move |_| {
                                                let pagination = pagination
                                                    .get()
                                                    .clone()
                                                    .reset_page()
                                                    .set_my_feed(true);
                                                set_pagination.set(pagination);
                                            }
                                        >
                                            "Your Feed"
                                        </button>
                                    </li>
                                </Show>
                                <li class="nav-item">
                                    <button
                                        class="nav-link"
                                        class:active=move || { !pagination.get().get_my_feed() }
                                        on:click=move |_| {
                                            let pagination = pagination
                                                .get()
                                                .clone()
                                                .reset_page()
                                                .set_my_feed(false);
                                            set_pagination.set(pagination);
                                        }
                                    >
                                        "Global Feed"
                                    </button>
                                </li>
                            </ul>
                        </div>

                        <Transition fallback=|| view! { <p>"Loading articles"</p> }>
                            // {move || {
                            //     articles_res
                            //         .get()
                            //         .unwrap_or(ArticlesResponse::default())
                            //         .articles
                            //         .map(move |articles| {
                            //             let (articles, _) = signal(articles.articles);
                            //             view! {
                            //                 <ArticlePreviewList username=username articles=articles />
                            //             }
                            //         })
                            // }}
                            <ArticlePreviewList articles=articles />
                        </Transition>
                    </div>

                    <div class="col-md-3">
                        <div class="sidebar">
                            <p>"Popular Tags"</p>
                            <Transition fallback=|| view! { <p>"Loading popular tags"</p> }>
                                <TagList />
                            </Transition>
                        </div>
                    </div>

                    <ul class="pagination">
                        <For
                            each=move || pages()
                            key=|x| *x
                            children=move |x| {
                                let active = x == pagination.get().get_page();
                                view! {
                                    <li class="page-item" class:active=move || active>
                                        <button
                                            class="page-link"
                                            on:click=move |_| {
                                                let pagination = pagination.get().clone().set_page(x);
                                                set_pagination.set(pagination);
                                            }
                                        >
                                            {x}
                                        </button>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TagList() -> impl IntoView {
    let (tags, set_tags) = signal::<Vec<String>>(vec![]);

    let tags_fetcher = move || Tag::load_tags();
    let tags_res = LocalResource::new(tags_fetcher);


    // TODO: This is not the right way.
    Effect::new(move || {
        let tags_res_opt = tags_res.get();
        if let Some(tags_res) = tags_res_opt.as_deref() {
            set_tags.set(tags_res.tags.clone());
        } else {
            set_tags.set(vec![]);
        }
    });

    view! {
        <div class="tag-list">
            <Suspense fallback=move || view! { <p>"Loading Tags"</p> }>
                <ErrorBoundary fallback=|_| {
                    view! { <p class="error-messages text-xs-center">"Something went wrong."</p> }
                }>
                    <For
                        each=move || tags.get().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(_, t): (usize, String)| {
                            view! {
                                <a class="tag-pill tag-default" href="">
                                    {t}
                                </a>
                            }
                        }
                    />
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
