CREATE DATABASE "walletregistesystem";
CREATE TABLE wallets(
	id VARCHAR(255) PRIMARY KEY NOT NULL,
	cert TEXT NOT NULL,
	info jsonb NOT NULL,
	create_time timestamp NOT NULL,
    update_time timestamp NOT NULL
	);
//插入数据
INSERT INTO wallets (id,cert,info,create_time,update_time) VALUES ("varchar", "text", "jsonb",now(),now());
