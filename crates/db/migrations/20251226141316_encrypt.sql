-- migrate:up
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION encrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    encrypted text;
BEGIN
    key := current_setting('encryption.root_key', true);

    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            encrypted := pgp_sym_encrypt(data, key, 'compress-algo=1, cipher-algo=aes256');
            RETURN encrypted;
        EXCEPTION WHEN others THEN
            RETURN SQLERRM;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION decrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    decrypted text;
BEGIN
    key := current_setting('encryption.root_key', true);

    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            decrypted := pgp_sym_decrypt(data::bytea, key);
            RETURN decrypted;
        EXCEPTION WHEN others THEN
            RETURN data;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- migrate:down
DROP FUNCTION IF EXISTS decrypt_text(text);
DROP FUNCTION IF EXISTS encrypt_text(text);
DROP EXTENSION IF EXISTS pgcrypto;