import React, { useState } from "https://esm.sh/react@19.1.0";
import { createRoot } from "https://esm.sh/react-dom@19.1.0/client";

const h = React.createElement;

const SCENARIOS = [
  {
    id: "api",
    label: "GET /api/users",
    badge: "L7 Route → API",
    clientIp: "203.0.113.5",
    conn1: "HTTPS · TLS 1.3 · TCP #1",
    proxySteps: [
      "Terminates HTTPS (SSL cert lives here)",
      "Matches location /api/*",
      "Adds X-Forwarded-For: 203.0.113.5",
    ],
    conn2: "HTTP · Keep-Alive · TCP #2",
    target: "api",
    backendNote: "Socket sees proxy IP 10.0.0.10 — reads real user IP from trusted X-Forwarded-For.",
  },
  {
    id: "static",
    label: "GET /static/app.js",
    badge: "Cache / Static Route",
    clientIp: "203.0.113.5",
    conn1: "HTTPS · TLS 1.3 · TCP #1",
    proxySteps: [
      "Terminates HTTPS & checks proxy cache",
      "Cache MISS → routes /static/* to static pool",
      "Compresses response with gzip / Brotli",
    ],
    conn2: "HTTP · Keep-Alive · TCP #2",
    target: "static",
    backendNote: "Next visitor gets a fast Cache HIT straight from the reverse proxy — zero backend load.",
  },
  {
    id: "spoof",
    label: "Spoofed Header",
    badge: "Security Gotcha",
    clientIp: "198.51.100.99",
    conn1: "HTTPS + fake X-Forwarded-For: 1.2.3.4",
    proxySteps: [
      "Client sends fake header: 1.2.3.4",
      "Proxy appends true socket IP → 1.2.3.4, 198.51.100.99",
      "Backend MUST trust only the rightmost proxy-added IP",
    ],
    conn2: "HTTP · TCP #2 (from 10.0.0.10)",
    target: "api",
    backendNote: "If your backend blindly trusts the leftmost IP (1.2.3.4), attackers can bypass IP rate limits.",
  },
];

function ArrowConnector({ label, active }) {
  return h(
    "div",
    { className: `rdiag-connector${active ? " is-active" : ""}` },
    h("div", { className: "rdiag-line" }),
    h("span", { className: "rdiag-edge-label" }, label),
    h(
      "svg",
      {
        className: "rdiag-arrow-head",
        width: 14,
        height: 14,
        viewBox: "0 0 14 14",
        fill: "none",
        "aria-hidden": "true",
      },
      h("path", {
        d: "M7 1v10m0 0-4-4m4 4 4-4",
        stroke: "currentColor",
        strokeWidth: "1.6",
        strokeLinecap: "round",
        strokeLinejoin: "round",
      })
    )
  );
}

function ReverseProxyDiagram() {
  const [scenarioId, setScenarioId] = useState("api");
  const active = SCENARIOS.find((s) => s.id === scenarioId) || SCENARIOS[0];

  return h(
    "div",
    { className: "rdiag-card", role: "region", "aria-label": "Interactive Reverse Proxy Diagram" },
    // Top bar with scenario selector tabs
    h(
      "div",
      { className: "rdiag-toolbar" },
      h("span", { className: "rdiag-caption" }, "Interactive Flow"),
      h(
        "div",
        { className: "rdiag-tabs", role: "tablist" },
        SCENARIOS.map((s) =>
          h(
            "button",
            {
              key: s.id,
              type: "button",
              role: "tab",
              "aria-selected": s.id === active.id,
              className: `rdiag-tab${s.id === active.id ? " active" : ""}`,
              onClick: () => setScenarioId(s.id),
            },
            s.label
          )
        )
      )
    ),

    // Stage 1: Client
    h(
      "div",
      { className: "rdiag-flow" },
      h(
        "div",
        { className: "rdiag-node rdiag-node-active" },
        h(
          "div",
          { className: "rdiag-node-head" },
          h("span", { className: "rdiag-node-title" }, "Client Browser"),
          h("span", { className: "rdiag-pill" }, `IP ${active.clientIp}`)
        ),
        h(
          "p",
          { className: "rdiag-node-sub" },
          "Knows only example.com — never sees internal backend IPs."
        )
      ),

      h(ArrowConnector, { label: active.conn1, active: true }),

      // Stage 2: Reverse Proxy
      h(
        "div",
        { className: "rdiag-node rdiag-node-proxy" },
        h(
          "div",
          { className: "rdiag-node-head" },
          h("span", { className: "rdiag-node-title" }, "Reverse Proxy (NGINX / Envoy)"),
          h("span", { className: "rdiag-pill" }, "10.0.0.10")
        ),
        h(
          "ul",
          { className: "rdiag-steps" },
          active.proxySteps.map((step, idx) => h("li", { key: idx }, step))
        )
      ),

      h(ArrowConnector, { label: active.conn2, active: true }),

      // Stage 3: Private Backend Pool
      h(
        "div",
        { className: "rdiag-backends" },
        h(
          "div",
          {
            className: `rdiag-node rdiag-backend${active.target === "api" ? " rdiag-node-active" : " rdiag-node-dim"}`,
          },
          h(
            "div",
            { className: "rdiag-node-head" },
            h("span", { className: "rdiag-node-title" }, "API Server"),
            h("span", { className: "rdiag-pill" }, "10.0.0.20:8080")
          ),
          h("p", { className: "rdiag-node-sub" }, "Handles /api/* requests")
        ),
        h(
          "div",
          {
            className: `rdiag-node rdiag-backend${active.target === "static" ? " rdiag-node-active" : " rdiag-node-dim"}`,
          },
          h(
            "div",
            { className: "rdiag-node-head" },
            h("span", { className: "rdiag-node-title" }, "Static Server"),
            h("span", { className: "rdiag-pill" }, "10.0.0.21:8081")
          ),
          h("p", { className: "rdiag-node-sub" }, "Handles /static/* assets")
        )
      )
    ),

    // Bottom insight callout
    h(
      "div",
      { className: "rdiag-footer" },
      h("span", { className: "rdiag-badge" }, active.badge),
      h("span", { className: "rdiag-note" }, active.backendNote)
    )
  );
}

const DIAGRAMS = {
  "reverse-proxy": ReverseProxyDiagram,
};

document.querySelectorAll("[data-diagram]").forEach((el) => {
  const name = el.getAttribute("data-diagram");
  const Component = DIAGRAMS[name];
  if (Component) {
    createRoot(el).render(h(Component));
  }
});
