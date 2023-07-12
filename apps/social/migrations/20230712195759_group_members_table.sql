CREATE TABLE group_members (
    group_id CHAR(12) NOT NULL,
    user_id CHAR(12) NOT NULL,
    PRIMARY KEY (group_id, user_id),
    FOREIGN KEY (group_id) REFERENCES groups (id)
);
