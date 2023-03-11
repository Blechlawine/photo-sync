FROM fedora

# from https://nivethan.dev/devlog/cross-compiling-rust-gtk-projects-for-windows.html

WORKDIR /root
RUN dnf -y update
RUN dnf clean all
RUN dnf install -y git cmake file gcc make man sudo tar
RUN dnf install -y gcc-c++ boost boost-devel

# Build peldd
RUN git clone https://github.com/gsauthof/pe-util
WORKDIR pe-util
RUN git submodule update --init
RUN mkdir build

WORKDIR build
RUN cmake .. -DCMAKE_BUILD_TYPE=Release
RUN make

RUN mv /root/pe-util/build/peldd /usr/bin/peldd
RUN chmod +x /usr/bin/peldd


# Add package.sh
ADD package.sh /usr/bin/package.sh
RUN chmod +x /usr/bin/package.sh

# Install windows libraries
RUN dnf install -y mingw64-gcc 
RUN dnf install -y mingw64-freetype 
RUN dnf install -y mingw64-cairo 
RUN dnf install -y mingw64-harfbuzz 
RUN dnf install -y mingw64-pango 
RUN dnf install -y mingw64-poppler 
RUN dnf install -y mingw64-gtk3 
RUN dnf install -y mingw64-winpthreads-static 
RUN dnf install -y mingw64-glib2-static 

# Install rust
RUN useradd -ms /bin/bash rustacean
USER rustacean

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN /home/rustacean/.cargo/bin/rustup update

# Set up rust for cross-compiling
RUN /home/rustacean/.cargo/bin/rustup target add x86_64-pc-windows-gnu
ADD cargo.config /home/rustacean/.cargo/config
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/sys-root/mingw/lib/pkgconfig/
ENV GTK_INSTALL_PATH=/usr/x86_64-w64-mingw32/sys-root/mingw/

# Setup mount point
VOLUME /home/rustacean/src
WORKDIR /home/rustacean/src

# Build and package executable
CMD ["/usr/bin/package.sh"]