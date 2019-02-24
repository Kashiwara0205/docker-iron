FROM rust:1.32.0
ENV USER test_user
ADD iron_learning ./iron_learning
WORKDIR iron_learning
RUN cargo update
EXPOSE 4025