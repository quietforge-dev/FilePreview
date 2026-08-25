CREATE TABLE workspace_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    last_opened_at INTEGER NOT NULL
);

CREATE INDEX idx_workspace_history_opened
ON workspace_history(last_opened_at DESC, id DESC);

CREATE TABLE file_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    last_opened_at INTEGER NOT NULL
);

CREATE INDEX idx_file_history_opened
ON file_history(last_opened_at DESC, id DESC);
