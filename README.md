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

## 📦 Folder Structure

backend/
│
├── routes/ # Auth and ticket routes
├── models/ # User and ticket models
├── auth/ # JWT, password hashing
├── email/ # IMAP fetcher & SMTP auto-replies
├── cli/ # Interactive agent CLI
├── db.rs # PostgreSQL pool setup
├── main.rs # Axum server + email listener

yaml
Copy
Edit

---

## ⚙️ Setup

### 1. 📂 Clone the Repo

---bash
git clone https://github.com/yourusername/ticketing-system.git
cd ticketing-system/backend
2. 📦 Install Dependencies
bash
Copy
Edit
cargo build
3. 🛠️ Set up .env
Create a .env file in the root:

env
Copy
Edit
DATABASE_URL=postgres://postgres:password@localhost/ticketing
IMAP_SERVER=imap.yourdomain.com
IMAP_USER=support@yourdomain.com
IMAP_PASS=your-email-password
SMTP_USER=support@yourdomain.com
SMTP_PASS=your-smtp-password
SMTP_SERVER=smtp.yourdomain.com
4. 🗄️ Create Database
sql
Copy
Edit
-- Connect to PostgreSQL
CREATE DATABASE ticketing;

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'agent',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create tickets table
CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Open',
    priority TEXT NOT NULL DEFAULT 'Medium',
    customer_email TEXT,
    assigned_to UUID REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
🧪 Run Backend Server
bash
Copy
Edit
cargo run
✅ Server will start and the email watcher will poll for new emails every 30 seconds.

🖥️ Using the CLI
From cli/ folder:

bash
Copy
Edit
cargo run
You'll be prompted to:

Register or Login

View ticket menu (Create / List / Update / Delete)

📤 Test Email Integration
Send an email to your configured inbox

Wait 30s for it to be picked up

Check logs: a new ticket should be created

You should receive an auto-reply to your email

