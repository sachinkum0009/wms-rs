-- Initial migration for WMS database
-- This creates a basic health check table to verify migrations are working

CREATE TABLE IF NOT EXISTS health_check (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ok',
    checked_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert a sample record
INSERT INTO health_check (name, status) VALUES ('migration_test', 'ok') 
ON CONFLICT DO NOTHING;