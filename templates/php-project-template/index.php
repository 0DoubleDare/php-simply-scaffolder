<?php
require_once "./config/database.php";

$dsn = "mysql:host={$db_host};dbname={$db_name};charset=${charset}";

if $override_db_port != 0 {
    $dsn .= ";port=${$override_db_port}"
}

$pdo = new PDO($dsn, $user, $password);