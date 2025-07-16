# Automated_mail

# 🧾 SaaS Customer Support Ticketing System

A full-stack Rust-based customer support ticketing system featuring:

- JWT-based Authentication
- Role-based Access for Agents and Admins
- Ticket Creation, Assignment, and Status Management
- Email Integration (IMAP to Ticket + SMTP Auto-Reply)
- Interactive CLI for Agents
- Axum + SQLx + PostgreSQL backend

---

## 🚀 Features

### 👥 User Management
- Agent/Admin registration and login
- JWT token-based authentication
- Role-based access control

### 🎟️ Ticketing System
- Create, update, delete, list tickets
- Assign tickets to agents
- Status: `Open`, `In Progress`, `Pending`, `Resolved`, `Closed`
- Priority: `High`, `Medium`, `Low`

### ✉️ Email Integration
- Incoming email (IMAP) auto-creates tickets
- Auto-replies to customer emails (SMTP)
- Background email watcher using `tokio::spawn`

### 🖥️ Command Line Interface (CLI)
- Register / Login from terminal
- Interactive menu to create, update, list, and delete tickets

---

## 🧱 Tech Stack

- **Backend:** Rust, Axum, SQLx, PostgreSQL
- **Email:** `imap`, `lettre`
- **Async runtime:** Tokio
- **CLI:** `reqwest`, `serde`, `uuid`

---



