# PT register check

[![ci](https://github.com/yswtrue/pt_register_check/actions/workflows/main.yml/badge.svg)](https://github.com/yswtrue/pt_register_check/actions/workflows/main.yml)

PT 开放注册检测，每小时检测一次，如果检测有开放注册就会发送到指定邮箱

## 使用方法

修改`docker-compose.yml`中的环境变量，然后执行如下命令行便可以了

```
docker-compose up -d
# or
docker compose up -d
```