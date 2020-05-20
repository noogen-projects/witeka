CREATE DATABASE witeka_test CHARACTER SET utf8;
CREATE USER IF NOT EXISTS 'witeka_test'@'localhost' IDENTIFIED BY 'witeka_test';
GRANT ALL PRIVILEGES ON witeka_test.* TO 'witeka_test'@'localhost';