# 開発環境

## Rust

```
rustup override set nightly
```

## Diesel

```
cargo install diesel_cli
```

```
echo 'DATABASE_URL=mysql://root:@localhost:3306/rust' > .env
```

## MySQL

Ubuntu16.04にaptでMySQL5.7を入れる時にrootを空パスワードにするとどうなるか
http://qiita.com/mwatanabe@github/items/7e9a40d31bc27ab9d901

```
ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY '';
```
