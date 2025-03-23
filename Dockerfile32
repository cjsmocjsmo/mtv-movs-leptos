# Use the nginx:bookworm image as the base image
FROM arm32v7/nginx:bookworm

# Copy the contents of the dist directory to the nginx html directory
COPY ./dist/*.html /usr/share/nginx/html/
COPY ./dist/*.js /usr/share/nginx/html/
COPY ./dist/*.wasm /usr/share/nginx/html/

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]