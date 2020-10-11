apt-get update -y;
apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common;

curl -o https://download.docker.com/linux/ubuntu/dists/focal/pool/stable/amd64/docker-ce_19.03.13~3-0~ubuntu-focal_amd64.deb;
dpkg -i ./docker-ce_19.03.13~3-0~ubuntu-focal_amd64.deb;

usermod -aG docker $USER;
newgrp docker;

systemctl start docker.service;
curl -L "https://github.com/docker/compose/releases/download/1.27.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose;
chmod +x /usr/local/bin/docker-compose;

# docker-compose --version;
docker ps;
