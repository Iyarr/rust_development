
docker build --no-cache -t rust-app .
docker run -it --rm -d -p 7878:7878 --name rust-app rust-app

# docker stop rust-app
