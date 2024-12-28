import React, { lazy, memo, Suspense, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useParams } from "react-router";
import { marked } from "marked";

import TagsList from "../../features/tags/TagsList.jsx";
import { articlePageUnloaded, getArticle } from "@/reducers/article.js";
import ArticleMeta from "./ArticleMeta.jsx";
import { Link } from "react-router-dom"
import { favoriteArticle, unfavoriteArticle } from "@/reducers/articleList.js"
import { selectIsAuthenticated } from "@/features/auth/authSlice.js"

const CommentSection = lazy(
  () =>
    import(
      /* webpackChunkName: "CommentSection", webpackPrefetch: true  */ "../../features/comments/CommentSection.jsx"
    ),
);

const FAVORITED_CLASS = 'btn btn-sm btn-primary';
const NOT_FAVORITED_CLASS = 'btn btn-sm btn-outline-primary';

/**
 * Show one article with its comments
 *
 * @param {import('react-router-dom').RouteComponentProps<{ slug: string }>} props
 * @example
 * <Article />
 */
function Article({ match }) {
  const dispatch = useDispatch();
  const article = useSelector((state) => state.article.article);
  const inProgress = useSelector((state) => state.article.inProgress);
  const isAuthenticated = useSelector(selectIsAuthenticated);
  const { slug } = useParams();
  const renderMarkdown = () => ({
    __html: marked(article.body, { sanitize: true }),
  });

  const handleClick = (event) => {
    event.preventDefault();

    if (article.favorited) {
      dispatch(unfavoriteArticle(article.slug));
    } else {
      dispatch(favoriteArticle(article.slug));
    }
  };

  useEffect(() => {
    const fetchArticle = dispatch(getArticle(slug));
    return () => {
      fetchArticle.abort();
    };
  }, [match]);

  useEffect(() => () => dispatch(articlePageUnloaded()), []);

  if (!article) {
    return (
      <div className="article-page">
        <div className="container page">
          <div className="row article-content">
            <div className="col-xs-12">
              {inProgress && <h1 role="alert">Article is loading</h1>}
            </div>
          </div>
        </div>
      </div>
    );
  }

  const favoriteButtonClass = article.favorited
    ? FAVORITED_CLASS
    : NOT_FAVORITED_CLASS;

  return (
    <div className="article-page">
      <div className="banner">
        <div className="container">
          <h1>{article.title}</h1>
          <ArticleMeta />
        </div>
      </div>

      <div className="container page">
        <div className="row article-content">
          <div className="col-xs-12">
            <article dangerouslySetInnerHTML={renderMarkdown()} />

            <TagsList tags={article.tagList} />
          </div>
        </div>

        <hr />

        <div className="article-actions">
          <div className="article-meta">
            <Link to={`/profile/${article.author.username}`}>
              <img
                src={
                  article.author.image ||
                  'https://static.productionready.io/images/smiley-cyrus.jpg'
                }
                alt={article.author.username}
              />
            </Link>

            <div className="info">
              <Link className="author" to={`/profile/${article.author.username}`}>
                {article.author.username}
              </Link>
              <time className="date" dateTime={article.createdAt}>
                {new Date(article.createdAt).toDateString()}
              </time>
            </div>

            {isAuthenticated && (
              <button className={favoriteButtonClass} onClick={handleClick}>
                <i className="ion-heart" /> {article.favoritesCount}
              </button>
            )}
          </div>
        </div>

        <Suspense fallback={<p>Loading comments</p>}>
          <CommentSection />
        </Suspense>
      </div>
    </div>
  );
}

export default memo(Article);
