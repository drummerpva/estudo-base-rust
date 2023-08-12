CREATE TABLE IF NOT EXISTS likes (
  id int(10) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  created_at timestamp NOT NULL DEFAULT current_timestamp(),
  tweet_id int(10) UNSIGNED,
  FOREIGN KEY (tweet_id) REFERENCES tweets(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
