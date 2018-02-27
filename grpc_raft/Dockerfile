FROM rust

# Creating a directory to work from
RUN mkdir -p /tmp/src/raft
WORKDIR /tmp/src/raft

# Copy our app into that directory
COPY . /tmp/src/raft

# Build our app
CMD ["cargo", "build"]