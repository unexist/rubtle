 VERSION=0.0.2

 docker build -t unexist/rust:$VERSION .circleci/images/primary/
 docker login
 docker push unexist/rust:$VERSION
