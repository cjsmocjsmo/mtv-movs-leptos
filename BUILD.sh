git pull;
trunk build --release;
sudo docker run -d --name mtvmovslep -v /home/whitepi/mtv-movs-leptos/dist:/usr/share/nginx/html:ro -p 9000:80 nginx:bookworm;