WITH (
    'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB' AS program_id
)
SELECT
    dictGetOrDefault('DICTIONARY_SOLANA_TOKENS', 'type', program_id, '') as type,
    dictGetOrDefault('DICTIONARY_SOLANA_TOKENS', 'name', program_id, '') as name;
