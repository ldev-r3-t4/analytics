# analytics
The analytics server for the chat app

To run the server, install Rust and then run: `cargo run`

To build the docker container, install Docker and then run: `docker build -t analytics .`

To build and run containerized tests, run: `docker-compose -f docker-compose.test.yml up`

Note: The docker image created of the server is highly inefficient and non-minimal. Going forward,
intermediary images will be uploaded to Docker Hub to decrease the image build time and the generated
images will be shrunk so that it doesn't contain anything unecessary.
