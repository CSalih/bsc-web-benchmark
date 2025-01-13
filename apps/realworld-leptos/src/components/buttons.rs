use crate::auth;
use crate::components::ArticleSignal;
use leptos::prelude::*;
use log::error;

pub async fn follow_action(_other_user: String) -> Result<bool, ServerFnError> {
    Ok(false)
}

pub async fn fav_action(_slug: String) -> Result<bool, ServerFnError> {
    Ok(false)
}

#[component]
pub fn ButtonFollow(author: ReadSignal<String>, following: bool) -> impl IntoView {
    let auth_context = expect_context::<auth::AuthContext>();

    // let follow = create_server_action::<FollowAction>();
    let follow = Action::new(|username: &String| follow_action(username.to_string()));
    let result_call = follow.value();
    let follow_cond = move || {
        if let Some(x) = result_call.get() {
            match x {
                Ok(x) => x,
                Err(err) => {
                    error!("problem while following {err:?}");
                    following
                }
            }
        } else {
            following
        }
    };

    view! {
        <Show
            when=move || {
                auth_context.user.get().map(|x| x.username()).unwrap_or_default() != author.get()
            }
            fallback=|| ()
        >
            <form class="inline pull-xs-right">
                <input type="hidden" name="other_user" value=move || author.get() />
                <button type="submit" class="btn btn-sm btn-outline-secondary">
                    <Show
                        when=follow_cond
                        fallback=|| {
                            view! {
                                <i class="ion-plus-round"></i>
                                " Follow "
                            }
                        }
                    >
                        <i class="ion-close-round"></i>
                        " Unfollow "
                    </Show>
                    {move || author.get()}
                </button>
            </form>
        </Show>
    }
}

#[component]
pub fn ButtonFav(article: ArticleSignal) -> impl IntoView {
    let make_fav = Action::new(|slug: &String| fav_action(slug.to_string()));
    let result_make_fav = make_fav.value();
    let fav_count = move || {
        if let Some(x) = result_make_fav.get() {
            match x {
                Ok(result) => {
                    article.update(move |x| {
                        x.favorited = !x.favorited;
                        x.favorites_count =
                            (x.favorites_count + if result { 1 } else { -1 }).max(0);
                    });
                }
                Err(err) => {
                    error!("problem while fav {err:?}");
                }
            }
        }
        article.with(|x| x.favorites_count)
    };

    let has_fav = move || article.with(|x| x.favorited);

    view! {
        <div class="pull-xs-right">
            <button
                type="submit"
                class=move || {
                    if has_fav() {
                        "btn btn-sm btn-primary"
                    } else {
                        "btn btn-sm btn-outline-primary"
                    }
                }
            >
                <i class="ion-heart"></i>
                " "
                {fav_count}
            </button>
        </div>
    }
}
