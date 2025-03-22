# git pull;
# trunk build --release;
# sudo docker run -d -v /home/whitepi/mtv-movs-leptos/dist:/usr/share/nginx/html:ro -p 9091:80 nginx:bookworm;


# Navigate to the directory containing the Dockerfile
cd /home/whitepi/mtv-tv-leptos/

git pull;
trunk build --release;

# Build the Docker image
docker build -t mtvtvlep:latest .

# Run the Docker container
docker run -d -p 9092:80 mtvtvlep:latest