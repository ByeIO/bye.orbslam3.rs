# 映射wsl的podman到局域网
# 管理员身份运行

netsh interface portproxy add v4tov4 listenport=6901 listenaddress=0.0.0.0 connectport=6901 connectaddress=172.20.37.253