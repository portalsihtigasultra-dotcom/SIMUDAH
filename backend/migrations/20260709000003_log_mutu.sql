CREATE TABLE log_mutu (
    id BIGSERIAL PRIMARY KEY,
    data_id BIGINT NOT NULL REFERENCES data_curah_hujan(id) ON DELETE CASCADE,
    status_sebelum data_status NOT NULL,
    status_sesudah data_status NOT NULL,
    diubah_oleh BIGINT NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_log_mutu_data_id ON log_mutu(data_id);
CREATE INDEX idx_log_mutu_created_at ON log_mutu(created_at DESC);
