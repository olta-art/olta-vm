CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE processes (
    process_id VARCHAR(255) PRIMARY KEY,
    full_state JSONB NOT NULL,
    is_hot BOOLEAN DEFAULT true,
    last_activity TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    process_id VARCHAR(255) NOT NULL,
    collection_name VARCHAR(255) NOT NULL,
    doc_id VARCHAR(255) NOT NULL,
    document_data JSONB NOT NULL,
    synced_to_s3 BOOLEAN DEFAULT false,
    s3_txid VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(process_id, collection_name, doc_id)
);

CREATE INDEX idx_processes_hot ON processes(is_hot);
CREATE INDEX idx_processes_activity ON processes(last_activity);
CREATE INDEX idx_documents_process ON documents(process_id);
CREATE INDEX idx_documents_sync ON documents(synced_to_s3);
