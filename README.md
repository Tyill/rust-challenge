
### Как пользоваться:

1. Создать БД  и таблички в CLickHouse (CH далее), для этого вызвать скрипт:  
`./scripts/create_tables.sh`  
скрипт принимает пароль к CH, если установлен.

2. Отредактировать файл конфига, должен быть в месте запуска:  
`rust_challenge.json`
```
{
"db_url": "http://localhost:8123",
"db_name" : "stockdb",             // менять не надо (БД с таким названием создается в create_tables.sh) 
"db_user_name" : "default",
"db_user_password" : "alpha3"
}
```

3. Собрать исп файл и запустить:  
`cargo build`  
`./target/debug/rust_challenge`  
Выведет 10 последних статов юзеров.

### Аргументы командной строки для исп файла:

```
  -g, --gencount <GENCOUNT>    Number of generate transfer [default: 10]
  -s, --showcount <SHOWCOUNT>  Number of show stats [default: 10]
  -b, --beginsec <BEGINSEC>    Begin of time interval for stats [default: 1]
  -e, --endsec <ENDSEC>        End of time interval for stats [default: 1000]
  -c, --cngpath <CNGPATH>      Path of config file [default: rust_challenge.json]
  -h, --help                   Print help
  -V, --version                Print version
```

### Тесты

Тест проверяет только правильность расчетов статистики.  
Расположен здесь `scr/test.rs`

Для запуска набрать в консоле:
`cargo test`





