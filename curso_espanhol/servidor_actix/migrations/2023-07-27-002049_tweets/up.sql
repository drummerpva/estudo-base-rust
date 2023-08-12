CREATE TABLE IF NOT EXISTS tweets (
  id int(10) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  created_at timestamp DEFAULT current_timestamp(),
  message TEXT NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
