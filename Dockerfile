FROM rust:1.27

RUN apt-get update && apt-get install -y curl jq libssl-dev pkg-config time pigz psl && apt-get clean

RUN curl https://publicsuffix.org/list/public_suffix_list.dat -o public_suffix_list.dat
RUN curl -s https://opendata.rapid7.com/sonar.rdns_v2/ | \
    grep 'href="/sonar.rdns_v2/' | cut -d'"' -f2 > url.txt
RUN curl --location https://opendata.rapid7.com/`cat url.txt` \
    | pigz -dc | head -n 1M | jq -r .value > domains.txt

RUN apt-get update && apt-get -y install pypy && apt-get clean
RUN curl -O https://bootstrap.pypa.io/get-pip.py && pypy get-pip.py
RUN pypy -m pip install publicsuffix

# https://stackoverflow.com/a/38261124/1091116
ARG CACHE_DATE=not_a_date

ADD ./Cargo.toml .
ADD ./main.rs .
ENV RUSTFLAGS "-Ctarget-cpu=native"
RUN time cargo build --release --quiet

ADD ./main.py .

ADD ./bench.sh .

ENTRYPOINT ./bench.sh
