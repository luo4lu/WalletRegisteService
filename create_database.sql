CREATE DATABASE "walletregistesystem";
CREATE TABLE wallets(
	id VARCHAR(255) PRIMARY KEY NOT NULL,
	cert TEXT NOT NULL,
	info jsonb NOT NULL
	);
//插入数据
INSERT INTO wallets (id,cert,info) VALUES ("varchar", "text", "jsonb");
