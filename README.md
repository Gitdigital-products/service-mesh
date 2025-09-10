# service-mesh
service-mesh Istio-powered service mesh for Gitdigital.
# Service Mesh

Internal routing + service discovery for the **Gitdigital Products** ecosystem.  
Keeps track of registered services and lets others discover them.

## 🚀 Features
- Register new services with `POST /register`.
- Discover services with `GET /discover/:name`.
- Built with Axum + DashMap (in-memory registry).
- Lightweight mesh, expandable later into Envoy/Linkerd-style.

## 🛠️ Setup
```bash
cargo run
