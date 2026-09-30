# Kubernetes Deployment Guide (rust-learning)

This directory contains manifests for deploying all components to Kubernetes in the `rust-learning` namespace.

## Architecture & Endpoints

- **Web (FE):** `https://rust.NEXTECHINCUBATOR.COM` (Ingress -> Nginx SPA container)
- **API (Backend):** `https://rust-api.NEXTECHINCUBATOR.COM` (Ingress -> Axum API)
- **UAA (Keycloak):** `https://uaa.NEXTECHINCUBATOR.COM` (Ingress -> Keycloak 26)
- **Database (PostgreSQL 16):** `postgres.rust-learning.svc.cluster.local:5432` (StatefulSet with hostPath backed PV)
- **Executor (Worker):** `executor.rust-learning.svc.cluster.local:8082` (Internal Service)

## Prerequisites

- Ingress controller with `ingressClassName: nginx`
- cert-manager with `ClusterIssuer: letsencrypt-prod`

## Deployment Steps

### 1. Create Secrets

Create a secret file in `infrastructure/k8s/secrets/` (which is git-ignored) using `secrets.example.yaml` as a reference:

```bash
kubectl apply -f infrastructure/k8s/00-namespace.yaml
kubectl apply -f infrastructure/k8s/secrets/secrets.yaml
```

### 2. Apply Storage & Database

```bash
kubectl apply -f infrastructure/k8s/01-postgres-pv.yaml
kubectl apply -f infrastructure/k8s/02-postgres.yaml
```

### 3. Apply UAA / Keycloak

```bash
kubectl apply -f infrastructure/k8s/03-keycloak.yaml
```

### 4. Apply Executor & API

```bash
kubectl apply -f infrastructure/k8s/04-executor.yaml
kubectl apply -f infrastructure/k8s/05-api.yaml
```

### 5. Apply Web Frontend

```bash
kubectl apply -f infrastructure/k8s/06-web.yaml
```

Or apply all manifests together after creating namespace and secrets:

```bash
kubectl apply -f infrastructure/k8s/
```
