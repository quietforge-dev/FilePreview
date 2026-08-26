CREATE TABLE session_tabs (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK(kind IN ('workspace', 'file')),
    workspace_path TEXT NOT NULL,
    workspace_name TEXT NOT NULL,
    file_path TEXT,
    file_name TEXT,
    file_extension TEXT,
    current_directory TEXT,
    position INTEGER NOT NULL,
    active INTEGER NOT NULL DEFAULT 0 CHECK(active IN (0, 1)),
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_session_tabs_position
ON session_tabs(position ASC);
