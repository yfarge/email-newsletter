-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '82136371-83c0-4091-bdf1-e5d72ef109b1',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$Uc0cO+/9Vlab5CihMPFodQ$JehgJIduiSS+cVWkU8P9gS5dzt5oL8nnMXUMeiMcj8s'
);
