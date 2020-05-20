CREATE DATABASE witeka CHARACTER SET utf8;
CREATE USER IF NOT EXISTS 'witeka'@'localhost' IDENTIFIED BY 'witeka';
GRANT ALL PRIVILEGES ON witeka.* TO 'witeka'@'localhost';