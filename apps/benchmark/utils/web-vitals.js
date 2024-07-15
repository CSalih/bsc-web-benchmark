var p,
  w,
  W,
  S,
  D,
  X = -1,
  m = function (t) {
    addEventListener(
      "pageshow",
      function (n) {
        n.persisted && ((X = n.timeStamp), t(n));
      },
      !0,
    );
  },
  F = function () {
    return (
      window.performance &&
      performance.getEntriesByType &&
      performance.getEntriesByType("navigation")[0]
    );
  },
  L = function () {
    var t = F();
    return (t && t.activationStart) || 0;
  },
  d = function (t, n) {
    var r = F(),
      i = "navigate";
    return (
      X >= 0
        ? (i = "back-forward-cache")
        : r &&
          (document.prerendering || L() > 0
            ? (i = "prerender")
            : document.wasDiscarded
              ? (i = "restore")
              : r.type && (i = r.type.replace(/_/g, "-"))),
      {
        name: t,
        value: n === void 0 ? -1 : n,
        rating: "good",
        delta: 0,
        entries: [],
        id: "v3-"
          .concat(Date.now(), "-")
          .concat(Math.floor(8999999999999 * Math.random()) + 1e12),
        navigationType: i,
      }
    );
  },
  h = function (t, n, r) {
    try {
      if (PerformanceObserver.supportedEntryTypes.includes(t)) {
        var i = new PerformanceObserver(function (e) {
          Promise.resolve().then(function () {
            n(e.getEntries());
          });
        });
        return i.observe(Object.assign({ type: t, buffered: !0 }, r || {})), i;
      }
    } catch {}
  },
  s = function (t, n, r, i) {
    var e, a;
    return function (c) {
      n.value >= 0 &&
        (c || i) &&
        ((a = n.value - (e || 0)) || e === void 0) &&
        ((e = n.value),
        (n.delta = a),
        (n.rating = (function (u, o) {
          return u > o[1] ? "poor" : u > o[0] ? "needs-improvement" : "good";
        })(n.value, r)),
        t(n));
    };
  },
  x = function (t) {
    requestAnimationFrame(function () {
      return requestAnimationFrame(function () {
        return t();
      });
    });
  },
  A = function (t) {
    var n = function (r) {
      (r.type !== "pagehide" && document.visibilityState !== "hidden") || t(r);
    };
    addEventListener("visibilitychange", n, !0),
      addEventListener("pagehide", n, !0);
  },
  B = function (t) {
    var n = !1;
    return function (r) {
      n || (t(r), (n = !0));
    };
  },
  g = -1,
  j = function () {
    return document.visibilityState !== "hidden" || document.prerendering
      ? 1 / 0
      : 0;
  },
  C = function (t) {
    document.visibilityState === "hidden" &&
      g > -1 &&
      ((g = t.type === "visibilitychange" ? t.timeStamp : 0), rn());
  },
  H = function () {
    addEventListener("visibilitychange", C, !0),
      addEventListener("prerenderingchange", C, !0);
  },
  rn = function () {
    removeEventListener("visibilitychange", C, !0),
      removeEventListener("prerenderingchange", C, !0);
  },
  O = function () {
    return (
      g < 0 &&
        ((g = j()),
        H(),
        m(function () {
          setTimeout(function () {
            (g = j()), H();
          }, 0);
        })),
      {
        get firstHiddenTime() {
          return g;
        },
      }
    );
  },
  E = function (t) {
    document.prerendering
      ? addEventListener(
          "prerenderingchange",
          function () {
            return t();
          },
          !0,
        )
      : t();
  },
  N = [1800, 3e3],
  Y = function (t, n) {
    (n = n || {}),
      E(function () {
        var r,
          i = O(),
          e = d("FCP"),
          a = h("paint", function (c) {
            c.forEach(function (u) {
              u.name === "first-contentful-paint" &&
                (a.disconnect(),
                u.startTime < i.firstHiddenTime &&
                  ((e.value = Math.max(u.startTime - L(), 0)),
                  e.entries.push(u),
                  r(!0)));
            });
          });
        a &&
          ((r = s(t, e, N, n.reportAllChanges)),
          m(function (c) {
            (e = d("FCP")),
              (r = s(t, e, N, n.reportAllChanges)),
              x(function () {
                (e.value = performance.now() - c.timeStamp), r(!0);
              });
          }));
      });
  },
  q = [0.1, 0.25],
  an = function (t, n) {
    (n = n || {}),
      Y(
        B(function () {
          var r,
            i = d("CLS", 0),
            e = 0,
            a = [],
            c = function (o) {
              o.forEach(function (f) {
                if (!f.hadRecentInput) {
                  var l = a[0],
                    k = a[a.length - 1];
                  e &&
                  f.startTime - k.startTime < 1e3 &&
                  f.startTime - l.startTime < 5e3
                    ? ((e += f.value), a.push(f))
                    : ((e = f.value), (a = [f]));
                }
              }),
                e > i.value && ((i.value = e), (i.entries = a), r());
            },
            u = h("layout-shift", c);
          u &&
            ((r = s(t, i, q, n.reportAllChanges)),
            A(function () {
              c(u.takeRecords()), r(!0);
            }),
            m(function () {
              (e = 0),
                (i = d("CLS", 0)),
                (r = s(t, i, q, n.reportAllChanges)),
                x(function () {
                  return r();
                });
            }),
            setTimeout(r, 0));
        }),
      );
  },
  T = { passive: !0, capture: !0 },
  on = /* @__PURE__ */ new Date(),
  J = function (t, n) {
    p ||
      ((p = n),
      (w = t),
      (W = /* @__PURE__ */ new Date()),
      $(removeEventListener),
      Z());
  },
  Z = function () {
    if (w >= 0 && w < W - on) {
      var t = {
        entryType: "first-input",
        name: p.type,
        target: p.target,
        cancelable: p.cancelable,
        startTime: p.timeStamp,
        processingStart: p.timeStamp + w,
      };
      S.forEach(function (n) {
        n(t);
      }),
        (S = []);
    }
  },
  cn = function (t) {
    if (t.cancelable) {
      var n =
        (t.timeStamp > 1e12 ? /* @__PURE__ */ new Date() : performance.now()) -
        t.timeStamp;
      t.type == "pointerdown"
        ? (function (r, i) {
            var e = function () {
                J(r, i), c();
              },
              a = function () {
                c();
              },
              c = function () {
                removeEventListener("pointerup", e, T),
                  removeEventListener("pointercancel", a, T);
              };
            addEventListener("pointerup", e, T),
              addEventListener("pointercancel", a, T);
          })(n, t)
        : J(n, t);
    }
  },
  $ = function (t) {
    ["mousedown", "keydown", "touchstart", "pointerdown"].forEach(function (n) {
      return t(n, cn, T);
    });
  },
  U = [100, 300],
  un = function (t, n) {
    (n = n || {}),
      E(function () {
        var r,
          i = O(),
          e = d("FID"),
          a = function (o) {
            o.startTime < i.firstHiddenTime &&
              ((e.value = o.processingStart - o.startTime),
              e.entries.push(o),
              r(!0));
          },
          c = function (o) {
            o.forEach(a);
          },
          u = h("first-input", c);
        (r = s(t, e, U, n.reportAllChanges)),
          u &&
            A(
              B(function () {
                c(u.takeRecords()), u.disconnect();
              }),
            ),
          u &&
            m(function () {
              var o;
              (e = d("FID")),
                (r = s(t, e, U, n.reportAllChanges)),
                (S = []),
                (w = -1),
                (p = null),
                $(addEventListener),
                (o = a),
                S.push(o),
                Z();
            });
      });
  },
  nn = 0,
  I = 1 / 0,
  b = 0,
  fn = function (t) {
    t.forEach(function (n) {
      n.interactionId &&
        ((I = Math.min(I, n.interactionId)),
        (b = Math.max(b, n.interactionId)),
        (nn = b ? (b - I) / 7 + 1 : 0));
    });
  },
  tn = function () {
    return D ? nn : performance.interactionCount || 0;
  },
  dn = function () {
    "interactionCount" in performance ||
      D ||
      (D = h("event", fn, {
        type: "event",
        buffered: !0,
        durationThreshold: 0,
      }));
  },
  _ = [200, 500],
  en = 0,
  z = function () {
    return tn() - en;
  },
  v = [],
  M = {},
  G = function (t) {
    var n = v[v.length - 1],
      r = M[t.interactionId];
    if (r || v.length < 10 || t.duration > n.latency) {
      if (r) r.entries.push(t), (r.latency = Math.max(r.latency, t.duration));
      else {
        var i = { id: t.interactionId, latency: t.duration, entries: [t] };
        (M[i.id] = i), v.push(i);
      }
      v.sort(function (e, a) {
        return a.latency - e.latency;
      }),
        v.splice(10).forEach(function (e) {
          delete M[e.id];
        });
    }
  },
  sn = function (t, n) {
    (n = n || {}),
      E(function () {
        var r;
        dn();
        var i,
          e = d("INP"),
          a = function (u) {
            u.forEach(function (l) {
              l.interactionId && G(l),
                l.entryType === "first-input" &&
                  !v.some(function (k) {
                    return k.entries.some(function (R) {
                      return (
                        l.duration === R.duration && l.startTime === R.startTime
                      );
                    });
                  }) &&
                  G(l);
            });
            var o,
              f = ((o = Math.min(v.length - 1, Math.floor(z() / 50))), v[o]);
            f &&
              f.latency !== e.value &&
              ((e.value = f.latency), (e.entries = f.entries), i());
          },
          c = h("event", a, {
            durationThreshold:
              (r = n.durationThreshold) !== null && r !== void 0 ? r : 40,
          });
        (i = s(t, e, _, n.reportAllChanges)),
          c &&
            ("PerformanceEventTiming" in window &&
              "interactionId" in PerformanceEventTiming.prototype &&
              c.observe({ type: "first-input", buffered: !0 }),
            A(function () {
              a(c.takeRecords()),
                e.value < 0 && z() > 0 && ((e.value = 0), (e.entries = [])),
                i(!0);
            }),
            m(function () {
              (v = []),
                (en = tn()),
                (e = d("INP")),
                (i = s(t, e, _, n.reportAllChanges));
            }));
      });
  },
  K = [2500, 4e3],
  P = {},
  vn = function (t, n) {
    (n = n || {}),
      E(function () {
        var r,
          i = O(),
          e = d("LCP"),
          a = function (o) {
            var f = o[o.length - 1];
            f &&
              f.startTime < i.firstHiddenTime &&
              ((e.value = Math.max(f.startTime - L(), 0)),
              (e.entries = [f]),
              r());
          },
          c = h("largest-contentful-paint", a);
        if (c) {
          r = s(t, e, K, n.reportAllChanges);
          var u = B(function () {
            P[e.id] ||
              (a(c.takeRecords()), c.disconnect(), (P[e.id] = !0), r(!0));
          });
          ["keydown", "click"].forEach(function (o) {
            addEventListener(
              o,
              function () {
                return setTimeout(u, 0);
              },
              !0,
            );
          }),
            A(u),
            m(function (o) {
              (e = d("LCP")),
                (r = s(t, e, K, n.reportAllChanges)),
                x(function () {
                  (e.value = performance.now() - o.timeStamp),
                    (P[e.id] = !0),
                    r(!0);
                });
            });
        }
      });
  },
  Q = [800, 1800],
  ln = function t(n) {
    document.prerendering
      ? E(function () {
          return t(n);
        })
      : document.readyState !== "complete"
        ? addEventListener(
            "load",
            function () {
              return t(n);
            },
            !0,
          )
        : setTimeout(n, 0);
  },
  pn = function (t, n) {
    n = n || {};
    var r = d("TTFB"),
      i = s(t, r, Q, n.reportAllChanges);
    ln(function () {
      var e = F();
      if (e) {
        var a = e.responseStart;
        if (a <= 0 || a > performance.now()) return;
        (r.value = Math.max(a - L(), 0)),
          (r.entries = [e]),
          i(!0),
          m(function () {
            (r = d("TTFB", 0)), (i = s(t, r, Q, n.reportAllChanges))(!0);
          });
      }
    });
  };
const mn = "unknown",
  V = "http://localhost:8000/api/v1/event";
function y(t) {
  const n = JSON.stringify({
    type: "web-vitals/v1",
    project: mn,
    timestamp: /* @__PURE__ */ new Date().toISOString(),
    href: location ? location.href : null,
    metadata: {
      referrer: document ? document.referrer : null,
      userAgent: navigator ? navigator.userAgent : null,
      memory: navigator ? navigator.deviceMemory : void 0,
      cpus: navigator ? navigator.hardwareConcurrency : void 0,
      webdriver: navigator ? navigator.webdriver : void 0,
      connection:
        navigator != null && navigator.connection
          ? {
              type: navigator.connection.type,
              downlink: navigator.connection.downlink,
              downlinkMax: navigator.connection.downlinkMax,
              effectiveType: navigator.connection.effectiveType,
              saveData: navigator.connection.saveData,
              rtt: navigator.connection.rtt,
            }
          : void 0,
    },
    data: t,
  });
  navigator.sendBeacon
    ? navigator.sendBeacon(V, n)
    : fetch(V, {
        body: n,
        method: "POST",
        keepalive: !0,
        headers: { "Content-Type": "application/json" },
      });
}
pn(y);
Y(y);
an(y);
un(y);
vn(y);
sn(y);
