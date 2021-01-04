CREATE TABLE POLLEDDATA (
	id INT PRIMARY KEY,
	oid VARCHAR (50)  NOT NULL,
	ip_addr VARCHAR (50) NOT NULL,
	community VARCHAR (25)  NOT NULL
);

INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100001, '1.3.6.1.2.1.2.2.1.10.1', '192.168.1.1:161',  'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100002, '1.3.6.1.2.1.2.2.1.10.2', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100003, '1.3.6.1.2.1.2.2.1.10.3', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100004, '1.3.6.1.2.1.2.2.1.10.4',  '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100005, '1.3.6.1.2.1.2.2.1.10.5', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100006, '1.3.6.1.2.1.2.2.1.10.6', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100007, '1.3.6.1.2.1.2.2.1.10.7', '192.168.1.1:161',  'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100008, '1.3.6.1.2.1.2.2.1.10.8', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100009, '1.3.6.1.2.1.2.2.1.10.9', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100010, '1.3.6.1.2.1.2.2.1.10.10',  '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100011, '1.3.6.1.2.1.2.2.1.10.11', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100012, '1.3.6.1.2.1.2.2.1.10.12', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100013, '1.3.6.1.2.1.2.2.1.16.1', '192.168.1.1:161',  'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100014, '1.3.6.1.2.1.2.2.1.16.2', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100015, '1.3.6.1.2.1.2.2.1.16.3', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100016, '1.3.6.1.2.1.2.2.1.16.4',  '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100017, '1.3.6.1.2.1.2.2.1.16.5', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100018, '1.3.6.1.2.1.2.2.1.16.6', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100019, '1.3.6.1.2.1.2.2.1.16.7', '192.168.1.1:161',  'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100020, '1.3.6.1.2.1.2.2.1.16.8', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100021, '1.3.6.1.2.1.2.2.1.16.9', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100022, '1.3.6.1.2.1.2.2.1.16.10',  '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100023, '1.3.6.1.2.1.2.2.1.16.11', '192.168.1.1:161', 'paraam') ;
INSERT INTO POLLEDDATA(id,oid,ip_addr,community) VALUES(100024, '1.3.6.1.2.1.2.2.1.16.12', '192.168.1.1:161', 'paraam') ;
CREATE TABLE STATSDATA (
	id INT,
	timestamp TIMESTAMP,
	vtype INT,
	value Double Precision
);