-- migrate:up
CREATE ROLE readonly LOGIN ENCRYPTED PASSWORD '****************';
GRANT SELECT ON ALL TABLES IN SCHEMA public TO readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO readonly;

-- migrate:down
DROP ROLE IF EXISTS readonly;