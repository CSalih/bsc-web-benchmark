import React, { memo } from 'react';
import { Link } from 'react-router-dom';

import { selectIsAuthenticated, selectUser } from '../features/auth/authSlice';

/**
 * App footer
 *
 * @example
 * <Footer />
 */
function Footer() {
  return (
    <footer>
      <div className="container">
        <Link name="global-feed" class="logo-font" to="/"> conduit </Link>
        <span className="attribution">
        An interactive learning project from
        <a href="https://thinkster.io">Thinkster</a>. Code &amp; design licensed
        under MIT.
      </span>
      </div>
    </footer>
  );
}

export default memo(Footer);
