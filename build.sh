apt-get update -y;
apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common;

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -;

add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu focal stable";
apt update -y;
apt install -y docker-ce;


curl -L "https://github.com/docker/compose/releases/download/1.27.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose;
chmod +x /usr/local/bin/docker-compose;

docker-compose --version;
docker ps;
