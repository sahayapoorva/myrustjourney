---
title: "Reverse Proxy Explained"
date: "2026-11-22"
---

A **reverse proxy** is a server that sits *in front* of your backend servers and handles incoming requests from clients on their behalf. To a user's browser, the reverse proxy *looks* like the website itself — the client never knows which internal server actually generated the response.

<div data-diagram="reverse-proxy"></div>

## Forward Proxy vs. Reverse Proxy

People mix these up all the time because the only difference is **who the proxy represents**:

| Proxy Type | Sits in front of | Hides | Common examples |
| :--- | :--- | :--- | :--- |
| **Forward proxy** | Clients (users) | Client IPs from destination servers | Corporate firewalls, VPNs |
| **Reverse proxy** | Backend servers | Server topology & IPs from clients | NGINX, Envoy, HAProxy, Cloudflare |

## What a Reverse Proxy Actually Does

Even in front of a *single* backend app, a reverse proxy solves several real-world headaches in one place:

- **TLS / SSL termination**: Decrypts incoming HTTPS traffic at the edge so your SSL certificates live in one place and backend apps don't burn CPU on encryption (for strict zero-trust networks, the proxy can re-encrypt proxy → backend traffic with TLS/mTLS).
- **Routing (L4 vs. L7)**: A **Layer 4** proxy routes purely on IP and TCP/UDP port (fast, blind to HTTP). A **Layer 7** proxy understands HTTP/gRPC, so it can route `/api/*` to one service and `/static/*` to another.
- **Caching & compression**: Caches public responses and compresses payloads (gzip/Brotli) before sending them to the browser, cutting both backend load and latency.
- **Connection buffering & pooling**: Buffers slow/flaky mobile clients (shielding backends from Slowloris-style attacks) while reusing warm keep-alive connections to backends.

A minimal NGINX reverse proxy config looks like this:

```nginx
server {
    listen 443 ssl;
    server_name example.com;

    location /api/ {
        proxy_pass http://10.0.0.20:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Reverse Proxy vs. Load Balancer vs. API Gateway

These tools overlap heavily, so it helps to separate their primary roles:
- **Reverse proxy**: Fronts one or more backends (routing, TLS, caching, hiding internal servers).
- **Load balancer**: Specifically distributes traffic across *multiple* backend instances.
- **API gateway**: Adds API-level concerns like authentication, rate limiting, and request/response transformation.

## Production Gotchas to Watch Out For

1. **Your backend loses the client's real IP at the TCP layer**: An L7 reverse proxy creates **two separate TCP connections** (`Client ↔ Proxy` and `Proxy ↔ Backend`). Your backend's raw socket sees the *proxy's* IP, not the user's. The proxy passes the real IP via headers like `X-Forwarded-For` — but because attackers can spoof HTTP headers, only trust the IP appended by your *own* proxy.
2. **A single reverse proxy is a Single Point of Failure (SPOF)**: Putting one proxy in front of 10 healthy backends means if that one proxy crashes, all 10 backends become unreachable. Production setups run **multiple stateless proxy instances** (behind DNS, Anycast, or a cloud L4 load balancer) paired with **health checks** to stop routing to dead backends.
3. **Never accidentally cache private responses**: Caching public pages is fast, but caching authenticated user data without proper `Cache-Control: private` headers can leak one user's response to another.

## Key Takeaways

- A forward proxy protects **clients**; a reverse proxy protects and abstracts **servers**.
- Because L7 reverse proxies terminate two separate TCP connections, always handle `X-Forwarded-For` carefully instead of blindly trusting client headers.
- Run at least two reverse proxy instances with backend health checks so the proxy itself doesn't become a Single Point of Failure.

## References

- [Cloudflare Learning Center — What is a Reverse Proxy?](https://www.cloudflare.com/learning/cdn/glossary/reverse-proxy/)
- [NGINX Admin Guide — Reverse Proxy](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/)
