ALTER TABLE to_do_items DROP constraint to_do_items_title_key;
ALTER TABLE to_do_items RENAME COLUMN title TO name;
ALTER TABLE to_do_items DROP COLUMN id;
ALTER TABLE to_do_items ADD PRIMARY KEY (name);
