-- Your SQL goes here
CREATE TABLE personas (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR(60) NOT NULL,
  `lastname` VARCHAR(60) NULL,
  `country` VARCHAR(100) NULL,
  `province` VARCHAR(100)  NULL,
  `email` VARCHAR(100) NOT NULL,
  phone VARCHAR(60) NULL,
  sex VARCHAR(1) NOT NULL,
  comment1 TEXT NULL,
  comment2 TEXT NULL,
  comment3 TEXT NULL,
  age INT(11) NOT NULL,
  mozilla_news INT(1) NOT NULL
)