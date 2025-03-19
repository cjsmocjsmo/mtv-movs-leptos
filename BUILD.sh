git pull;
trunk build --release;
sudo docker run -d -v /home/teresa/mtvleptos/dist:/usr/share/nginx/html:ro -p 9000:8080 nginx:bookworm;