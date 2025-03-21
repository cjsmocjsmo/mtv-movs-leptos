# Use the nginx:bookworm image as the base image
FROM nginx:bookworm

# Copy the contents of the dist directory to the nginx html directory
COPY /home/whitepi/mtv-movs-leptos/dist/*.html /usr/share/nginx/html/
COPY /home/whitepi/mtv-movs-leptos/dist/*.js /usr/share/nginx/html/
COPY /home/whitepi/mtv-movs-leptos/dist/*.wasm /usr/share/nginx/html/

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]