git pull;
trunk build --release;
sudo docker run -d -v /home/whitepi/mtv-movs-leptos/dist:/usr/share/nginx/html:ro -p 9091:80 nginx:bookworm;