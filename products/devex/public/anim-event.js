/*! AnimEvent v1.0.16 (c) anseki https://github.com/anseki/anim-event */
var AnimEvent = (function (n) {
  var e = {};
  function t(r) {
    if (e[r]) return e[r].exports;
    var o = (e[r] = { i: r, l: !1, exports: {} });
    return n[r].call(o.exports, o, o.exports, t), (o.l = !0), o.exports;
  }
  return (
    (t.m = n),
    (t.c = e),
    (t.d = function (n, e, r) {
      t.o(n, e) || Object.defineProperty(n, e, { enumerable: !0, get: r });
    }),
    (t.r = function (n) {
      "undefined" !== typeof Symbol &&
        Symbol.toStringTag &&
        Object.defineProperty(n, Symbol.toStringTag, { value: "Module" }),
        Object.defineProperty(n, "__esModule", { value: !0 });
    }),
    (t.t = function (n, e) {
      if ((1 & e && (n = t(n)), 8 & e)) return n;
      if (4 & e && "object" === typeof n && n && n.__esModule) return n;
      var r = Object.create(null);
      if (
        (t.r(r),
        Object.defineProperty(r, "default", { enumerable: !0, value: n }),
        2 & e && "string" !== typeof n)
      )
        for (var o in n)
          t.d(
            r,
            o,
            function (e) {
              return n[e];
            }.bind(null, o)
          );
      return r;
    }),
    (t.n = function (n) {
      var e =
        n && n.__esModule
          ? function () {
              return n.default;
            }
          : function () {
              return n;
            };
      return t.d(e, "a", e), e;
    }),
    (t.o = function (n, e) {
      return Object.prototype.hasOwnProperty.call(n, e);
    }),
    (t.p = ""),
    t((t.s = 0))
  );
})([
  function (n, e, t) {
    "use strict";
    t.r(e);
    var r = 500,
      o = [],
      i =
        window.requestAnimationFrame ||
        window.mozRequestAnimationFrame ||
        window.webkitRequestAnimationFrame ||
        window.msRequestAnimationFrame ||
        function (n) {
          return setTimeout(n, 1e3 / 60);
        },
      u =
        window.cancelAnimationFrame ||
        window.mozCancelAnimationFrame ||
        window.webkitCancelAnimationFrame ||
        window.msCancelAnimationFrame ||
        function (n) {
          return clearTimeout(n);
        },
      a = Date.now(),
      l = void 0;
    function c() {
      var n = void 0,
        e = void 0;
      l && (u.call(window, l), (l = null)),
        o.forEach(function (e) {
          var t;
          (t = e.event) && ((e.event = null), e.listener(t), (n = !0));
        }),
        n ? ((a = Date.now()), (e = !0)) : Date.now() - a < r && (e = !0),
        e && (l = i.call(window, c));
    }
    function f(n) {
      var e = -1;
      return (
        o.some(function (t, r) {
          return t.listener === n && ((e = r), !0);
        }),
        e
      );
    }
    var d = {
      add: function (n) {
        var e = void 0;
        return -1 === f(n)
          ? (o.push((e = { listener: n })),
            function (n) {
              (e.event = n), l || c();
            })
          : null;
      },
      remove: function (n) {
        var e;
        (e = f(n)) > -1 &&
          (o.splice(e, 1), !o.length && l && (u.call(window, l), (l = null)));
      },
    };
    e.default = d;
  },
]).default;
