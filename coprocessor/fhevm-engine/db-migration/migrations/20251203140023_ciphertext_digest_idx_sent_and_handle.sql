-- Track when entries are created for fair queuing of unsent transactions.
ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS created_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Handle SELECTTs on handle only by the txn-sender.
CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_handle
ON ciphertext_digest (handle);

-- Handle SELECTTs on unsent txns with limited retries by the txn-sender.
CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_unsent
ON ciphertext_digest (txn_is_sent, created_at);
