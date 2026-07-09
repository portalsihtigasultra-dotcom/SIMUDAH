CREATE TABLE pos_curah_hujan (
    id BIGSERIAL PRIMARY KEY,
    nama VARCHAR(255) NOT NULL,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    elevasi INTEGER,
    tipe_alat VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE data_curah_hujan (
    id BIGSERIAL PRIMARY KEY,
    pos_id BIGINT NOT NULL REFERENCES pos_curah_hujan(id) ON DELETE RESTRICT,
    tanggal DATE NOT NULL,
    nilai_mm DOUBLE PRECISION NOT NULL,
    jam_pengamatan TIME,
    petugas_id BIGINT NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status_mutu data_status NOT NULL DEFAULT 'mentah',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_curah_hujan_pos_id ON data_curah_hujan(pos_id);
CREATE INDEX idx_data_curah_hujan_tanggal ON data_curah_hujan(tanggal);
CREATE INDEX idx_data_curah_hujan_status_mutu ON data_curah_hujan(status_mutu);
