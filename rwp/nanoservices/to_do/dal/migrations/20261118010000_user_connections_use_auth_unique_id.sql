ALTER TABLE user_connections
    ALTER COLUMN user_id TYPE TEXT USING user_id::TEXT;

ALTER TABLE user_connections
    ADD CONSTRAINT user_connections_to_do_id_fkey
    FOREIGN KEY (to_do_id) REFERENCES to_do_items (id) ON DELETE CASCADE;
