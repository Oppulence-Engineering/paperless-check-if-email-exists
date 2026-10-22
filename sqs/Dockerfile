FROM messense/rust-musl-cross:x86_64-musl AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Rust project files
COPY . .

# Build the release binary for the musl target
RUN cargo build --release --bin reacher_sqs --target x86_64-unknown-linux-musl

# Install dependencies for ChromeDriver
FROM public.ecr.aws/lambda/provided:al2023 AS runtime

ARG CHROME_VERSION=156.0.8078.47

# Install dependencies
# From: https://github.com/umihico/docker-selenium-lambda/blob/73b511330db1d4d160264b00e0f25521de6b0615/Dockerfile
RUN dnf install -y unzip && \
    curl --fail --show-error -Lo "/tmp/chromedriver-linux64.zip" "https://storage.googleapis.com/chrome-for-testing-public/${CHROME_VERSION}/linux64/chromedriver-linux64.zip" && \
    curl --fail --show-error -Lo "/tmp/chrome-linux64.zip" "https://storage.googleapis.com/chrome-for-testing-public/${CHROME_VERSION}/linux64/chrome-linux64.zip" && \
    unzip /tmp/chromedriver-linux64.zip -d /opt/ && \
    unzip /tmp/chrome-linux64.zip -d /opt/
RUN dnf install -y atk cups-libs gtk3 libXcomposite alsa-lib \
    libXcursor libXdamage libXext libXi libXrandr libXScrnSaver \
    libXtst pango at-spi2-atk libXt xorg-x11-server-Xvfb \
    xorg-x11-xauth dbus-glib dbus-glib-devel nss mesa-libgbm

# Copy the statically linked binary to the runtime's expected location
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/reacher_sqs /var/runtime/bootstrap
# The config file must be as ./backend_config.toml
COPY --from=builder /usr/src/app/backend/backend_config.toml .

# Set the Lambda handler (entry point)
CMD ["/var/runtime/bootstrap"]
