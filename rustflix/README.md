# Rustflix CLI

A command-line tool for managing users, videos, and view records.

## Demo

https://github.com/user-attachments/assets/6f6ab4d4-c156-4404-85e4-8f2e96f72708

## Setup

### Prerequisites

- Rust and Cargo
- PostgreSQL 15+
- Diesel CLI

### Installation

```bash
# Build and install
cargo build --release && cargo install --path .

# Set up database (create .env file first)
echo 'DATABASE_URL=postgres://username:password@localhost:5432/rustflix' > .env

# Run migrations
diesel migration run
```

### macOS Library Path (if needed)

```bash
export LIBRARY_PATH="/opt/homebrew/opt/postgresql@15/lib:$LIBRARY_PATH"
```

---

## Commands

### User Commands

| Command                                    | Description             |
| ------------------------------------------ | ----------------------- |
| `rustflix user create <name> <email>`      | Create a new user       |
| `rustflix user update <id> <name> <email>` | Update an existing user |
| `rustflix user delete <id>`                | Delete a user           |
| `rustflix user show`                       | Show all users          |

**Examples:**

```bash
# Create a user
rustflix user create "John Doe" "john@example.com"

# Update user with ID 1
rustflix user update 1 "Jane Doe" "jane@example.com"

# Delete user with ID 1
rustflix user delete 1

# Show all users
rustflix user show
```

---

### Video Commands

| Command                                            | Description              |
| -------------------------------------------------- | ------------------------ |
| `rustflix video create <title> <description>`      | Create a new video       |
| `rustflix video update <id> <title> <description>` | Update an existing video |
| `rustflix video delete <id>`                       | Delete a video           |
| `rustflix video show`                              | Show all videos          |

**Examples:**

```bash
# Create a video
rustflix video create "My Video" "This is a great video"

# Update video with ID 1
rustflix video update 1 "Updated Title" "Updated description"

# Delete video with ID 1
rustflix video delete 1

# Show all videos
rustflix video show
```

---

### View Commands (Watch Records)

| Command                                                              | Description                          |
| -------------------------------------------------------------------- | ------------------------------------ |
| `rustflix view create <user_id> <video_id> <watch_start> <duration>` | Record a view                        |
| `rustflix view show`                                                 | Show all views                       |
| `rustflix view show-pretty`                                          | Show all views with user/video names |

**Examples:**

```bash
# Record a view (user 1 watched video 2 for 120 minutes)
rustflix view create 1 2 "2024-01-15T10:30:00" 120

# Show all views (raw)
rustflix view show

# Show all views with names
rustflix view show-pretty
```

---

## Help

```bash
# General help
rustflix --help

# Help for specific entity
rustflix user --help
rustflix video --help
rustflix view --help

# Help for specific command
rustflix user create --help
```
